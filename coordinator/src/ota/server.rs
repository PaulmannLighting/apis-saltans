use std::collections::BTreeMap;

use le_stream::ToLeStream;
use log::{debug, warn};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender, WeakSender};
use tokio::sync::oneshot;
use tokio::task::{AbortHandle, Id, JoinError, JoinHandle};
use zb_aps::apsde::{DataIndication, IndividualEndpoint, ReceivedDestination, Source};
use zb_core::destination::Device;
use zb_core::{Cluster, Direction, FullAddress, IeeeAddress};
use zb_hw::NcpHandle;
use zb_zcl::global::default_response::DefaultResponse;
use zb_zcl::ota_upgrade::{
    Command as OtaCommand, ImageBlockRequest, ImagePageRequest, QueryNextImageResponse,
    QueryResponse, QuerySpecificFileResponse, UpgradeEndRequest,
};
use zb_zcl::{
    Cluster as ZclCluster, Command, Frame, Scope, Status, UnsequencedFrame, UnsequencedHeader,
};

use super::state::RequestContext;
use super::transfer::{Replacement, Transfer, TransferExit, TransferMessage};
use super::{
    Image, Message, OTA_PROFILE, UpdateError, UpdateResult, reply_zcl,
    request_from_unsequenced_frame, zcl,
};

/// Handle used by the OTA server to route messages to one destination transfer.
#[derive(Debug)]
struct ActiveTransfer {
    target: FullAddress,
    messages: Sender<TransferMessage>,
    task: AbortHandle,
    task_id: Id,
}

/// Registered OTA subscription and its frame-forwarding task.
#[derive(Debug)]
struct ActiveSubscription {
    messages: Sender<zcl::SubscriptionMessage>,
    task: JoinHandle<()>,
}

#[derive(Debug)]
enum AddressResolver {
    Ncp(NcpHandle),
    #[cfg(test)]
    Fixed(IeeeAddress),
}

impl AddressResolver {
    async fn resolve(
        &self,
        short_id: zb_core::short_id::Device,
    ) -> Result<IeeeAddress, zb_hw::Error> {
        match self {
            Self::Ncp(ncp) => ncp.short_id_to_ieee_address(short_id).await,
            #[cfg(test)]
            Self::Fixed(address) => Ok(*address),
        }
    }
}

enum ServerEvent {
    Message(Message),
    Shutdown,
    Transfer(Result<(Id, TransferExit), JoinError>),
}

/// Stateful OTA Upgrade server actor.
#[derive(Debug)]
pub struct Server {
    addresses: AddressResolver,
    zcl: Sender<zcl::Message>,
    sender: WeakSender<ServerEvent>,
    inbound: Receiver<ServerEvent>,
    subscription: Option<ActiveSubscription>,
    transfers: BTreeMap<Device, ActiveTransfer>,
    update_task_limit: usize,
}

impl Server {
    /// Create an empty OTA server with a limit on concurrent destination transfer tasks.
    const fn new(
        addresses: AddressResolver,
        zcl: Sender<zcl::Message>,
        sender: WeakSender<ServerEvent>,
        inbound: Receiver<ServerEvent>,
        update_task_limit: usize,
    ) -> Self {
        Self {
            addresses,
            zcl,
            sender,
            inbound,
            subscription: None,
            transfers: BTreeMap::new(),
            update_task_limit,
        }
    }

    /// Process every OTA server event through one message inbox.
    pub async fn run(mut self) {
        let mut hardware_unavailable = false;
        while let Some(event) = self.inbound.recv().await {
            match event {
                ServerEvent::Transfer(result) => {
                    self.transfer_finished(result).await;
                }
                ServerEvent::Message(message) => match message {
                    Message::Update {
                        target,
                        target_endpoint,
                        source_endpoint,
                        image,
                        completion,
                    } => {
                        if hardware_unavailable {
                            let _result =
                                completion.send(Err(UpdateError::HardwareEventStreamClosed));
                        } else {
                            self.update(
                                target,
                                target_endpoint,
                                source_endpoint,
                                image,
                                completion,
                            )
                            .await;
                        }
                    }
                    Message::Received { indication } => {
                        if !hardware_unavailable {
                            self.received_ota(indication).await;
                        }
                    }
                    Message::HardwareUnavailable => {
                        hardware_unavailable = true;
                        self.stop_transfers_for_hardware_failure().await;
                    }
                },
                ServerEvent::Shutdown => break,
            }
            if hardware_unavailable && self.transfers.is_empty() {
                break;
            }
        }

        self.unsubscribe().await;

        for transfer in self.transfers.values() {
            transfer.task.abort();
        }
    }

    async fn stop_transfers_for_hardware_failure(&self) {
        let transfers = self
            .transfers
            .values()
            .map(|transfer| transfer.messages.clone())
            .collect::<Vec<_>>();
        for messages in transfers {
            if messages
                .send(TransferMessage::HardwareUnavailable)
                .await
                .is_err()
            {
                debug!("Failed to notify completed OTA transfer of hardware failure");
            }
        }
    }

    /// Spawn the OTA server actor and return its message handle.
    pub(crate) fn spawn(
        ncp: NcpHandle,
        zcl: Sender<zcl::Message>,
        update_task_limit: usize,
    ) -> Sender<Message> {
        let (sender, messages) = tokio::sync::mpsc::channel(crate::MPSC_CHANNEL_SIZE);
        let (events, inbound) = tokio::sync::mpsc::channel(crate::MPSC_CHANNEL_SIZE);
        let server = Self::new(
            AddressResolver::Ncp(ncp),
            zcl,
            events.downgrade(),
            inbound,
            update_task_limit,
        );
        spawn(forward_api_messages(messages, events));
        spawn(server.run());
        sender
    }

    /// Replace an existing destination update or admit a new destination transfer task.
    async fn update(
        &mut self,
        target: FullAddress,
        target_endpoint: IndividualEndpoint,
        source_endpoint: IndividualEndpoint,
        image: Image,
        completion: oneshot::Sender<UpdateResult>,
    ) {
        let destination = Device::new(target.short_id(), target_endpoint.get());
        let existing_transfer = self
            .transfers
            .get(&destination)
            .map(|transfer| transfer.messages.clone());
        if existing_transfer.is_none() && self.transfers.len() >= self.update_task_limit {
            let _result = completion.send(Err(UpdateError::UpdateTaskLimitReached {
                limit: self.update_task_limit,
            }));
            return;
        }
        if let Err(error) = self.ensure_subscription().await {
            let _result = completion.send(Err(error));
            return;
        }

        if let Some(messages) = existing_transfer {
            let replacement = TransferMessage::Replace(Box::new(Replacement {
                target,
                target_endpoint,
                source_endpoint,
                image,
                completion,
            }));
            match messages.send(replacement).await {
                Ok(()) => {
                    if let Some(transfer) = self.transfers.get_mut(&destination) {
                        transfer.target = target;
                    }
                    return;
                }
                Err(error) => {
                    self.transfers.remove(&destination);
                    let TransferMessage::Replace(replacement) = error.0 else {
                        unreachable!("the failed message remains an update replacement");
                    };
                    let Replacement {
                        target,
                        target_endpoint,
                        source_endpoint,
                        image,
                        completion,
                    } = *replacement;
                    self.start_transfer(
                        target,
                        target_endpoint,
                        source_endpoint,
                        image,
                        completion,
                    );
                    return;
                }
            }
        }

        self.start_transfer(target, target_endpoint, source_endpoint, image, completion);
    }

    /// Lazily register the OTA frame subscription before offering the first update.
    async fn ensure_subscription(&mut self) -> Result<(), UpdateError> {
        if self.subscription.is_some() {
            return Ok(());
        }

        let (subscription, frames) = super::subscription();
        let messages = frames.sender();
        self.zcl
            .send(zcl::Message::Subscribe { subscription })
            .await
            .map_err(|_| UpdateError::Subscription)?;
        let task = spawn(forward_subscription_frames(frames, self.sender.clone()));
        self.subscription = Some(ActiveSubscription { messages, task });
        Ok(())
    }

    /// Remove the OTA subscription when there are no active update offers.
    async fn unsubscribe_if_idle(&mut self) {
        if self.transfers.is_empty() {
            self.unsubscribe().await;
        }
    }

    /// Stop frame forwarding and unregister the active OTA subscription.
    async fn unsubscribe(&mut self) {
        let Some(subscription) = self.subscription.take() else {
            return;
        };
        subscription.task.abort();
        if self
            .zcl
            .send(zcl::Message::Unsubscribe {
                messages: subscription.messages,
            })
            .await
            .is_err()
        {
            warn!("Failed to unregister the OTA ZCL subscription");
        }
    }

    /// Spawn and register the sole destination task for a newly admitted update.
    fn start_transfer(
        &mut self,
        target: FullAddress,
        target_endpoint: IndividualEndpoint,
        source_endpoint: IndividualEndpoint,
        image: Image,
        completion: oneshot::Sender<UpdateResult>,
    ) {
        let destination = Device::new(target.short_id(), target_endpoint.get());
        let (messages, inbound) = tokio::sync::mpsc::channel(crate::MPSC_CHANNEL_SIZE);
        let transfer = Transfer::new(
            self.zcl.clone(),
            target,
            target_endpoint,
            source_endpoint,
            image,
            completion,
            inbound,
        );
        let task = spawn(transfer.run());
        let task_id = task.id();
        let abort = task.abort_handle();
        spawn(forward_transfer_completion(
            task,
            task_id,
            self.sender.clone(),
        ));
        self.transfers.insert(
            destination,
            ActiveTransfer {
                target,
                messages,
                task: abort,
                task_id,
            },
        );
    }

    /// Validate an inbound OTA frame and route its command to the matching destination task.
    async fn received_ota(&mut self, indication: DataIndication<Frame<OtaCommand>, (), ()>) {
        let metadata = indication.metadata();
        let source_endpoint = match metadata.destination() {
            ReceivedDestination::Network { endpoint, .. }
            | ReceivedDestination::Extended { endpoint, .. } => endpoint,
            ReceivedDestination::Broadcast { endpoint, .. } => {
                let Some(endpoint) = IndividualEndpoint::new(endpoint) else {
                    warn!("Discarding OTA command addressed to a non-individual local endpoint");
                    return;
                };
                endpoint
            }
            ReceivedDestination::Group(_) => {
                warn!("Discarding group-addressed OTA command");
                return;
            }
            ReceivedDestination::ExtendedWithoutEndpoint(_) => {
                warn!("Discarding OTA command addressed without a local endpoint");
                return;
            }
        };
        let Ok(profile) = metadata.profile().inspect_err(|profile_id| {
            warn!("Discarding OTA command with unknown profile {profile_id:#06x}");
        }) else {
            return;
        };
        if profile != OTA_PROFILE {
            warn!("Discarding OTA command with unsupported profile {profile}");
            return;
        }
        let source = metadata.source();
        let Source::Network {
            address: source_address,
            endpoint,
        } = source
        else {
            warn!("Discarding OTA command from non-network source: {source:?}");
            return;
        };
        let Ok(short_id) = source_address.as_u16().try_into().inspect_err(|node_id| {
            warn!("Discarding OTA command from invalid node ID {node_id:#06x}");
        }) else {
            return;
        };

        let (_, zcl_frame) = indication.into_parts();
        let (zcl_header, command) = zcl_frame.into_parts();
        let context = RequestContext {
            destination: Device::new(short_id, endpoint.get()),
            source_endpoint,
            sequence_number: zcl_header.seq(),
        };
        if is_server_command(&command) {
            debug!(
                "Ignoring server-to-client OTA command from {}",
                context.destination
            );
            return;
        }

        let Some((messages, target)) = self
            .transfers
            .get(&context.destination)
            .map(|transfer| (transfer.messages.clone(), transfer.target))
        else {
            self.reject_unauthorized(context, command).await;
            return;
        };
        let resolved_address = match self.addresses.resolve(short_id).await {
            Ok(address) => address,
            Err(error) => {
                warn!(
                    "Failed to resolve the IEEE address for OTA request source {short_id}: {error}"
                );
                self.reject_unauthorized(context, command).await;
                return;
            }
        };
        if resolved_address != target.ieee_address() {
            warn!(
                "Rejecting OTA request from {short_id}: resolved IEEE address {resolved_address} \
                 does not match scheduled target {}",
                target.ieee_address()
            );
            self.reject_unauthorized(context, command).await;
            return;
        }
        let request = TransferMessage::Request { context, command };
        if let Err(error) = messages.send(request).await {
            self.transfers.remove(&context.destination);
            let TransferMessage::Request { context, command } = error.0 else {
                unreachable!("the failed message remains an OTA request");
            };
            self.reject_unauthorized(context, command).await;
            self.unsubscribe_if_idle().await;
        }
    }

    /// Remove a completed task if it is still the registered task for its destination.
    async fn transfer_finished(&mut self, result: Result<(Id, TransferExit), JoinError>) {
        let completion = match result {
            Ok((task_id, exit)) => {
                self.remove_transfer(exit.destination, task_id);
                Some((exit.completion, exit.result))
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
                None
            }
        };
        self.unsubscribe_if_idle().await;
        if let Some((completion, result)) = completion {
            let _result = completion.send(result);
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
        let frame: UnsequencedFrame<bytes::Bytes> = match command {
            OtaCommand::QueryNextImageRequest(_) => UnsequencedFrame::from_command(
                QueryNextImageResponse::new(QueryResponse::NotAuthorized),
            ),
            OtaCommand::QuerySpecificFileRequest(_) => UnsequencedFrame::from_command(
                QuerySpecificFileResponse::new(QueryResponse::NotAuthorized),
            ),
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
        let request = request_from_unsequenced_frame(
            context.destination.into(),
            context.source_endpoint,
            OTA_PROFILE,
            Cluster::OtaUpgrade.as_u16(),
            frame,
        );
        let Some(()) = reply_zcl(&self.zcl, context.sequence_number, request).await else {
            return;
        };
    }
}

/// Forward public OTA API messages into the server's private event inbox.
async fn forward_api_messages(mut messages: Receiver<Message>, events: Sender<ServerEvent>) {
    while let Some(message) = messages.recv().await {
        let hardware_unavailable = matches!(&message, Message::HardwareUnavailable);
        if events.send(ServerEvent::Message(message)).await.is_err() {
            return;
        }
        if hardware_unavailable {
            messages.close();
            while let Some(message) = messages.recv().await {
                if let Message::Update { completion, .. } = message {
                    let _result = completion.send(Err(UpdateError::HardwareEventStreamClosed));
                }
            }
            events.closed().await;
            return;
        }
    }
    let _result = events.send(ServerEvent::Shutdown).await;
}

/// Forward subscribed OTA frames through the server's ordinary message inbox.
async fn forward_subscription_frames(
    mut frames: zcl::SubscriptionReceiver,
    sender: WeakSender<ServerEvent>,
) {
    while let Some(message) = frames.recv().await {
        let zcl::SubscriptionMessage { indication } = message;
        let (metadata, zcl_frame) = indication.into_parts();
        let (zcl_header, cluster) = zcl_frame.into_parts();
        let ZclCluster::OtaUpgrade(command) = cluster else {
            warn!("Discarding non-OTA command delivered by the OTA ZCL subscription");
            continue;
        };
        let Some(sender) = sender.upgrade() else {
            return;
        };
        let event = ServerEvent::Message(Message::Received {
            indication: DataIndication::new(metadata, Frame::new(zcl_header, command)),
        });
        if sender.send(event).await.is_err() {
            return;
        }
    }
}

/// Forward one destination task's terminal result into the server event inbox.
async fn forward_transfer_completion(
    task: JoinHandle<TransferExit>,
    task_id: Id,
    sender: WeakSender<ServerEvent>,
) {
    let result = task.await.map(|exit| (task_id, exit));
    let Some(sender) = sender.upgrade() else {
        return;
    };
    let _result = sender.send(ServerEvent::Transfer(result)).await;
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

fn default_response(request_command_id: u8, status: Status) -> UnsequencedFrame<bytes::Bytes> {
    let response = DefaultResponse::new(request_command_id, status.into());
    UnsequencedFrame::new(
        UnsequencedHeader::new(
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
        update_task_limit: usize,
    ) -> (Sender<Message>, Self) {
        Self::test_new_resolving(zcl, update_task_limit, super::TEST_IEEE_ADDRESS)
    }

    pub(super) fn test_new_resolving(
        zcl: Sender<zcl::Message>,
        update_task_limit: usize,
        resolved_ieee_address: IeeeAddress,
    ) -> (Sender<Message>, Self) {
        let (sender, messages) = tokio::sync::mpsc::channel(crate::MPSC_CHANNEL_SIZE);
        let (events, inbound) = tokio::sync::mpsc::channel(crate::MPSC_CHANNEL_SIZE);
        let server = Self::new(
            AddressResolver::Fixed(resolved_ieee_address),
            zcl,
            events.downgrade(),
            inbound,
            update_task_limit,
        );
        spawn(forward_api_messages(messages, events));
        (sender, server)
    }
}
