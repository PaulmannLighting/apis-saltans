use std::collections::BTreeMap;
use std::future::poll_fn;
use std::task::Poll;

use le_stream::ToLeStream;
use log::{debug, warn};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot;
use tokio::task::{Id, JoinError, JoinSet};
use zb_aps::Data;
use zb_core::destination::Device;
use zb_core::{Cluster, Direction};
use zb_nwk::Source;
use zb_zcl::global::default_response::DefaultResponse;
use zb_zcl::ota_upgrade::{
    Command as OtaCommand, ImageBlockRequest, ImagePageRequest, QueryNextImageResponse,
    QueryResponse, QuerySpecificFileResponse, UpgradeEndRequest,
};
use zb_zcl::{Command, Frame, Scope, Status};

use super::state::RequestContext;
use super::transfer::{Transfer, TransferExit, TransferMessage};
use super::{
    Image, Message, Metadata, OTA_PROFILE, Payload, UpdateError, UpdateResult, reply_zcl, zcl,
};

/// Handle used by the OTA server to route messages to one destination transfer.
#[derive(Debug)]
struct ActiveTransfer {
    messages: Sender<TransferMessage>,
    task_id: Id,
}

enum ServerEvent {
    Message(Option<Message>),
    Transfer(Option<Result<(Id, TransferExit), JoinError>>),
}

/// Stateful OTA Upgrade server actor.
#[derive(Debug)]
pub struct Server {
    zcl: Sender<zcl::Message>,
    inbound: Receiver<Message>,
    transfers: BTreeMap<Device, ActiveTransfer>,
    tasks: JoinSet<TransferExit>,
    update_task_limit: usize,
}

impl Server {
    /// Create an empty OTA server with a limit on concurrent destination transfer tasks.
    fn new(
        zcl: Sender<zcl::Message>,
        inbound: Receiver<Message>,
        update_task_limit: usize,
    ) -> Self {
        Self {
            zcl,
            inbound,
            transfers: BTreeMap::new(),
            tasks: JoinSet::new(),
            update_task_limit,
        }
    }

    /// Process update requests, route inbound commands, and reap destination transfer tasks.
    pub async fn run(mut self) {
        loop {
            let event = poll_fn(|context| {
                if !self.tasks.is_empty()
                    && let Poll::Ready(task) = self.tasks.poll_join_next_with_id(context)
                {
                    return Poll::Ready(ServerEvent::Transfer(task));
                }

                self.inbound.poll_recv(context).map(ServerEvent::Message)
            })
            .await;
            match event {
                ServerEvent::Transfer(Some(result)) => {
                    self.transfer_finished(result);
                }
                ServerEvent::Transfer(None) => {}
                ServerEvent::Message(message) => {
                    let Some(message) = message else {
                        break;
                    };
                    match message {
                        Message::Update {
                            target,
                            image,
                            completion,
                        } => self.update(target, image, completion).await,
                        Message::Received { source, frame } => {
                            self.received(source, frame).await;
                        }
                    }
                }
            }
        }
    }

    /// Spawn the OTA server actor with the given destination transfer-task limit.
    pub(crate) fn spawn(
        zcl: Sender<zcl::Message>,
        receiver: Receiver<Message>,
        update_task_limit: usize,
    ) {
        spawn(Self::new(zcl, receiver, update_task_limit).run());
    }

    /// Replace an existing destination update or admit a new destination transfer task.
    async fn update(
        &mut self,
        target: Device,
        image: Image,
        completion: oneshot::Sender<UpdateResult>,
    ) {
        if let Some(messages) = self
            .transfers
            .get(&target)
            .map(|transfer| transfer.messages.clone())
        {
            let replacement = TransferMessage::Replace { image, completion };
            match messages.send(replacement).await {
                Ok(()) => return,
                Err(error) => {
                    self.transfers.remove(&target);
                    let TransferMessage::Replace { image, completion } = error.0 else {
                        unreachable!("the failed message remains an update replacement");
                    };
                    self.start_transfer(target, image, completion);
                    return;
                }
            }
        }

        if self.transfers.len() >= self.update_task_limit {
            let _result = completion.send(Err(UpdateError::UpdateTaskLimitReached {
                limit: self.update_task_limit,
            }));
            return;
        }
        self.start_transfer(target, image, completion);
    }

    /// Spawn and register the sole destination task for a newly admitted update.
    fn start_transfer(
        &mut self,
        target: Device,
        image: Image,
        completion: oneshot::Sender<UpdateResult>,
    ) {
        let (messages, inbound) = tokio::sync::mpsc::channel(crate::MPSC_CHANNEL_SIZE);
        let transfer = Transfer::new(self.zcl.clone(), target, image, completion, inbound);
        let task = self.tasks.spawn(transfer.run());
        self.transfers.insert(
            target,
            ActiveTransfer {
                messages,
                task_id: task.id(),
            },
        );
    }

    /// Validate an inbound frame and route its command to the matching destination task.
    async fn received(&mut self, source: Source, frame: Data<Frame<OtaCommand>>) {
        let aps_header = frame.header();
        let Ok(endpoint) = aps_header.source_endpoint().inspect_err(|error| {
            warn!("Discarding OTA command with invalid source endpoint: {error:?}");
        }) else {
            return;
        };
        let Ok(profile) = aps_header.profile().inspect_err(|profile_id| {
            warn!("Discarding OTA command with unknown profile {profile_id:#06x}");
        }) else {
            return;
        };
        if profile != OTA_PROFILE {
            warn!("Discarding OTA command with unsupported profile {profile}");
            return;
        }
        let Ok(short_id) = source.node_id().try_into().inspect_err(|node_id| {
            warn!("Discarding OTA command from invalid node ID {node_id:#06x}");
        }) else {
            return;
        };

        let (_, zcl_frame) = frame.into_parts();
        let (zcl_header, command) = zcl_frame.into_parts();
        let context = RequestContext {
            destination: Device::new(short_id, endpoint),
            source_ieee_address: source.ieee_address(),
            sequence_number: zcl_header.seq(),
        };
        if is_server_command(&command) {
            debug!(
                "Ignoring server-to-client OTA command from {}",
                context.destination
            );
            return;
        }

        let Some(messages) = self
            .transfers
            .get(&context.destination)
            .map(|transfer| transfer.messages.clone())
        else {
            self.reject_unauthorized(context, command).await;
            return;
        };
        let request = TransferMessage::Request { context, command };
        if let Err(error) = messages.send(request).await {
            self.transfers.remove(&context.destination);
            let TransferMessage::Request { context, command } = error.0 else {
                unreachable!("the failed message remains an OTA request");
            };
            self.reject_unauthorized(context, command).await;
        }
    }

    /// Remove a completed task if it is still the registered task for its destination.
    fn transfer_finished(&mut self, result: Result<(Id, TransferExit), JoinError>) {
        match result {
            Ok((task_id, exit)) => {
                self.remove_transfer(exit.destination, task_id);
                let _result = exit.completion.send(exit.result);
            }
            Err(error) => {
                let task_id = error.id();
                if !error.is_cancelled() {
                    warn!("OTA destination transfer task failed: {error}");
                }
                let destination = self.transfers.iter().find_map(|(destination, transfer)| {
                    (transfer.task_id == task_id).then_some(*destination)
                });
                if let Some(destination) = destination {
                    self.remove_transfer(destination, task_id);
                }
            }
        }
    }

    /// Remove `destination` only when it still names `task_id`.
    fn remove_transfer(&mut self, destination: Device, task_id: Id) {
        let is_current = self
            .transfers
            .get(&destination)
            .is_some_and(|transfer| transfer.task_id == task_id);
        if is_current {
            self.transfers.remove(&destination);
        }
    }

    /// Reply to a request for which no destination transfer is active.
    async fn reject_unauthorized(&self, context: RequestContext, command: OtaCommand) {
        let payload = match command {
            OtaCommand::QueryNextImageRequest(_) => {
                QueryNextImageResponse::new(QueryResponse::NotAuthorized).into()
            }
            OtaCommand::QuerySpecificFileRequest(_) => {
                QuerySpecificFileResponse::new(QueryResponse::NotAuthorized).into()
            }
            OtaCommand::ImageBlockRequest(_) => {
                default_response(<ImageBlockRequest as Command>::ID, Status::NotAuthorized)
            }
            OtaCommand::ImagePageRequest(_) => {
                default_response(<ImagePageRequest as Command>::ID, Status::NotAuthorized)
            }
            OtaCommand::UpgradeEndRequest(_) => {
                default_response(<UpgradeEndRequest as Command>::ID, Status::NotAuthorized)
            }
            OtaCommand::ImageNotify(_)
            | OtaCommand::QueryNextImageResponse(_)
            | OtaCommand::ImageBlockResponse(_)
            | OtaCommand::UpgradeEndResponse(_)
            | OtaCommand::QuerySpecificFileResponse(_) => return,
        };
        let Some(response) = reply_zcl(
            &self.zcl,
            context.destination,
            OTA_PROFILE,
            context.sequence_number,
            payload,
        )
        .await
        else {
            return;
        };
        if let Err(error) = response.await {
            warn!("Failed to transmit unauthorized OTA response: {error}");
        }
    }
}

const fn is_server_command(command: &OtaCommand) -> bool {
    matches!(
        command,
        OtaCommand::ImageNotify(_)
            | OtaCommand::QueryNextImageResponse(_)
            | OtaCommand::ImageBlockResponse(_)
            | OtaCommand::UpgradeEndResponse(_)
            | OtaCommand::QuerySpecificFileResponse(_)
    )
}

fn default_response(request_command_id: u8, status: Status) -> Payload {
    let response = DefaultResponse::new(request_command_id, status.into());
    Payload::new(
        zb_hw::Metadata::new(OTA_PROFILE, Cluster::OtaUpgrade.as_u16()),
        Metadata::new(
            Scope::Global,
            Direction::ServerToClient,
            true,
            None,
            <DefaultResponse as Command>::ID,
        ),
        response.to_le_stream().collect(),
    )
}

#[cfg(test)]
impl Server {
    pub(super) fn test_new(
        zcl: Sender<zcl::Message>,
        inbound: Receiver<Message>,
        update_task_limit: usize,
    ) -> Self {
        Self::new(zcl, inbound, update_task_limit)
    }
}
