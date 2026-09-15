//! Transceiver to send and receive ZDP messages.

use std::collections::BTreeMap;

use bytes::Bytes;
use le_stream::ToLeStream;
use log::{debug, error, trace, warn};
use tokio::runtime::Handle;
use tokio::spawn;
use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::mpsc::{Receiver, Sender, WeakSender};
use tokio::task::AbortHandle;
use tokio::time::sleep;
use zb_aps::apsde::{DataIndication, DataRequest, NetworkAddress, ReceivedDestination};
use zb_core::FullAddress;
use zb_core::node::Descriptor;
use zb_core::short_id::Device;
use zb_hw::NcpHandle;
use zb_zdp::{Command, DeviceAndServiceDiscovery, DeviceAnnce, Frame};

pub use self::message::Message;
use self::server::{Server, ServerRequest, is_server_request};
use self::submission::CommunicationSubmission;
use crate::aps::Aps;
use crate::correlation::{
    Cancellation, Key, PROTOCOL_QUARANTINE_TIMEOUT, PROTOCOL_RESPONSE_TIMEOUT, Registry, Token,
};
use crate::event::EventSink;
use crate::response::ApsProtocolResponse;
use crate::{Device as DeviceEvent, Event, MPSC_CHANNEL_SIZE};

mod discovery;
mod match_desc;
mod message;
mod node_desc;
mod server;
mod submission;

const INITIAL_SERVER_OPERATION_ID: u64 = 0;
const INITIAL_COMMUNICATION_SUBMISSION_ID: u64 = 0;
const COMMUNICATION_SUBMISSION_LIMIT: usize = MPSC_CHANNEL_SIZE;
const SERVER_OPERATION_LIMIT: usize = MPSC_CHANNEL_SIZE;

/// Zigbee transceiver actor.
#[derive(Debug)]
pub struct Transceiver {
    server: Server,
    events: EventSink,
    responses: Registry<Command>,
    inbox: WeakSender<Message>,
    communication_submissions: BTreeMap<u64, CommunicationSubmission>,
    next_communication_submission_id: u64,
    server_operations: BTreeMap<u64, AbortHandle>,
    next_server_operation_id: u64,
}

/// Construction, startup, and actor-inbox processing.
impl Transceiver {
    /// Create a new transceiver.
    #[must_use]
    pub fn new(
        ncp: NcpHandle,
        aps: Aps,
        events: EventSink,
        descriptor: Descriptor,
        inbox: WeakSender<Message>,
    ) -> Self {
        Self {
            server: Server::new(ncp, aps, descriptor, inbox.clone()),
            events,
            responses: Registry::new(),
            inbox,
            communication_submissions: BTreeMap::new(),
            next_communication_submission_id: INITIAL_COMMUNICATION_SUBMISSION_ID,
            server_operations: BTreeMap::new(),
            next_server_operation_id: INITIAL_SERVER_OPERATION_ID,
        }
    }

    /// Start the ZDP transceiver.
    pub fn spawn(
        ncp: NcpHandle,
        aps: Aps,
        events: EventSink,
        descriptor: Descriptor,
    ) -> Sender<Message> {
        let (zdp_tx, zdp_rx) = tokio::sync::mpsc::channel(MPSC_CHANNEL_SIZE);
        spawn(Self::new(ncp, aps, events, descriptor, zdp_tx.downgrade()).run(zdp_rx));
        zdp_tx
    }

    /// Run the transceiver.
    pub async fn run(mut self, mut messages: Receiver<Message>) {
        while let Some(message) = messages.recv().await {
            if !self.handle_actor_message(message) {
                break;
            }
        }
        self.abort_server_operations();
        self.abort_communication_submissions();
    }

    fn handle_actor_message(&mut self, message: Message) -> bool {
        match message {
            Message::Received {
                indication,
                response_required,
            } => {
                self.handle_message_received(indication, response_required);
            }
            Message::NetworkDown => {
                self.abort_server_operations();
                self.handle_network_down();
            }
            Message::HardwareUnavailable => {
                self.abort_server_operations();
                self.fail_communication_submissions_for_hardware_unavailability();
                self.responses.hardware_unavailable();
                return false;
            }
            Message::Cancel { token } => {
                if self.responses.cancel(token) {
                    self.schedule_quarantine_timeout(token);
                }
            }
            Message::ResponseTimeout { token } => {
                if self.responses.timeout(token) {
                    self.schedule_quarantine_timeout(token);
                }
            }
            Message::QuarantineTimeout { token } => {
                self.responses.expire_quarantine(token);
            }
            Message::ReplyTransmissionFailed { error } => {
                error!("ZDP server response transmission failed: {error}");
            }
            Message::ServerOperationFinished { id } => {
                if self.server_operations.remove(&id).is_none() {
                    debug!("Ignoring completion for unknown ZDP server operation {id}");
                }
            }
            Message::CommunicationSubmissionFinished { id, result } => {
                let Some(submission) = self.communication_submissions.remove(&id) else {
                    debug!("Ignoring completion for unknown ZDP communication submission {id}");
                    return true;
                };
                let CommunicationSubmission {
                    token,
                    protocol_response,
                    response,
                    task: _,
                } = submission;
                let result = match result {
                    Ok(transmission) => Ok(ApsProtocolResponse::new(
                        transmission,
                        protocol_response,
                        self.cancellation(token),
                    )),
                    Err(error) => {
                        self.responses.discard(token);
                        Err(error)
                    }
                };
                response.send(result).unwrap_or_else(drop);
            }
            Message::Communicate {
                device,
                request,
                response,
            } => {
                self.communicate(device, request, response);
            }
        }
        true
    }
}

/// Inbound message routing and background ZDP server-operation management.
impl Transceiver {
    fn handle_message_received(
        &mut self,
        indication: DataIndication<Frame<Command>, (), ()>,
        response_required: bool,
    ) {
        let Some((source_address, key)) = received_key(&indication) else {
            return;
        };
        let request_was_broadcast = matches!(
            indication.metadata().destination(),
            ReceivedDestination::Broadcast { .. }
        );
        trace!("Received ZDP message: {indication:?}");
        let (_, zdp_frame) = indication.into_parts();
        let (seq, command) = zdp_frame.into_parts();

        if let Command::DeviceAndServiceDiscovery(DeviceAndServiceDiscovery::DeviceAnnce(
            device_annce,
        )) = &command
        {
            handle_device_annce(&self.events, device_annce.as_ref());
            return;
        }
        if is_server_request(&command) {
            if response_required {
                self.spawn_server_operation(ServerRequest::new(
                    source_address,
                    request_was_broadcast,
                    seq,
                    command,
                ));
            }
            return;
        }

        if self.responses.complete(key, command.clone()) {
            debug!(
                "Answering ZDP request: seq={seq} cluster_id={:#06X}",
                command.cluster_id()
            );
        } else if self.responses.release_quarantine(key) {
            debug!("Discarding late ZDP response with quarantined sequence {seq}");
        } else {
            warn!("Unexpected ZDP response: {command:?}");
        }
    }

    fn spawn_server_operation(&mut self, request: ServerRequest) {
        if self.server_operations.len() >= SERVER_OPERATION_LIMIT {
            warn!(
                "Discarding ZDP server request because the operation limit of \
                 {SERVER_OPERATION_LIMIT} has been reached"
            );
            return;
        }

        let id = self.allocate_server_operation_id();
        let server = self.server.clone();
        let inbox = self.inbox.clone();
        let task = spawn(async move {
            server.handle(request).await;
            let Some(inbox) = inbox.upgrade() else {
                return;
            };
            inbox
                .send(Message::ServerOperationFinished { id })
                .await
                .unwrap_or_else(|error| {
                    debug!("Failed to retire ZDP server operation: {error}");
                });
        });
        let previous = self.server_operations.insert(id, task.abort_handle());
        debug_assert!(previous.is_none());
    }

    fn abort_server_operations(&mut self) {
        for operation in std::mem::take(&mut self.server_operations).into_values() {
            operation.abort();
        }
    }

    fn allocate_server_operation_id(&mut self) -> u64 {
        loop {
            let id = self.next_server_operation_id;
            self.next_server_operation_id = self.next_server_operation_id.wrapping_add(1);
            if !self.server_operations.contains_key(&id) {
                return id;
            }
        }
    }
}

/// Outbound ZDP communication and APS submission lifecycle management.
impl Transceiver {
    /// Send a ZDP unicast message with back-channel communication.
    ///
    /// # Returns
    ///
    /// The result is returned through `response` after the request has been handed to the APS
    /// actor.
    fn communicate(
        &mut self,
        device: Device,
        request: DataRequest<Bytes>,
        response: tokio::sync::oneshot::Sender<Result<ApsProtocolResponse<Command>, crate::Error>>,
    ) {
        if self.communication_submissions.len() >= COMMUNICATION_SUBMISSION_LIMIT {
            warn!(
                "Rejecting ZDP communication because the submission limit of \
                 {COMMUNICATION_SUBMISSION_LIMIT} has been reached"
            );
            response
                .send(Err(crate::Error::SendError))
                .unwrap_or_else(drop);
            return;
        }
        let (seq, token, protocol_response) = match self
            .responses
            .register(|sequence| Key::from_zdp_command(device, sequence, &request))
        {
            Ok(registration) => registration,
            Err(error) => {
                response.send(Err(error)).unwrap_or_else(drop);
                return;
            }
        };
        self.schedule_response_timeout(token);
        let request = request.map_asdu(|payload| Frame::new(seq, payload).to_le_stream().collect());
        let aps = self.server.aps().clone();
        let inbox = self.inbox.clone();
        let id = self.allocate_communication_submission_id();
        let task = spawn(async move {
            let result = aps.transmit(request).await;
            let Some(inbox) = inbox.upgrade() else {
                return;
            };
            inbox
                .send(Message::CommunicationSubmissionFinished { id, result })
                .await
                .unwrap_or_else(|error| {
                    debug!("Failed to complete ZDP communication submission: {error}");
                });
        });
        let previous = self.communication_submissions.insert(
            id,
            CommunicationSubmission {
                token,
                protocol_response,
                response,
                task: task.abort_handle(),
            },
        );
        debug_assert!(previous.is_none());
    }

    fn handle_network_down(&mut self) {
        let submissions = std::mem::take(&mut self.communication_submissions);
        let protected = submissions
            .values()
            .map(|submission| submission.token)
            .collect::<Vec<_>>();
        let quarantined = self
            .responses
            .network_down_preserving(&zb_hw::TransmissionError::NoRoute, protected);
        for token in quarantined {
            self.schedule_quarantine_timeout(token);
        }

        for submission in submissions.into_values() {
            submission.task.abort();
            submission
                .response
                .send(Err(
                    zb_hw::Error::from(zb_hw::TransmissionError::NoRoute).into()
                ))
                .unwrap_or_else(drop);
        }
    }

    fn fail_communication_submissions_for_hardware_unavailability(&mut self) {
        for submission in std::mem::take(&mut self.communication_submissions).into_values() {
            submission.task.abort();
            submission
                .response
                .send(Err(zb_hw::Error::ActorUnavailable.into()))
                .unwrap_or_else(drop);
        }
    }

    fn abort_communication_submissions(&mut self) {
        for submission in std::mem::take(&mut self.communication_submissions).into_values() {
            submission.task.abort();
        }
    }

    fn allocate_communication_submission_id(&mut self) -> u64 {
        loop {
            let id = self.next_communication_submission_id;
            self.next_communication_submission_id =
                self.next_communication_submission_id.wrapping_add(1);
            if !self.communication_submissions.contains_key(&id) {
                return id;
            }
        }
    }
}

/// Pending-response cancellation, timeout, and quarantine lifecycle management.
impl Transceiver {
    fn cancellation(&self, token: Token) -> Cancellation {
        let inbox = self.inbox.clone();
        let runtime = Handle::current();
        Cancellation::new(token, move |token| {
            let Some(inbox) = inbox.upgrade() else {
                return;
            };
            match inbox.try_send(Message::Cancel { token }) {
                Ok(()) => {}
                Err(TrySendError::Full(message)) => {
                    runtime.spawn(async move {
                        inbox.send(message).await.unwrap_or_else(|error| {
                            debug!("Failed to enqueue ZDP response cancellation: {error}");
                        });
                    });
                }
                Err(TrySendError::Closed(_)) => {
                    debug!("Failed to enqueue ZDP response cancellation: actor unavailable");
                }
            }
        })
    }

    fn schedule_response_timeout(&self, token: Token) {
        let inbox = self.inbox.clone();
        spawn(async move {
            sleep(PROTOCOL_RESPONSE_TIMEOUT).await;
            let Some(inbox) = inbox.upgrade() else {
                return;
            };
            inbox
                .send(Message::ResponseTimeout { token })
                .await
                .unwrap_or_else(|error| {
                    debug!("Failed to enqueue ZDP response timeout: {error}");
                });
        });
    }

    fn schedule_quarantine_timeout(&self, token: Token) {
        let inbox = self.inbox.clone();
        spawn(async move {
            sleep(PROTOCOL_QUARANTINE_TIMEOUT).await;
            let Some(inbox) = inbox.upgrade() else {
                return;
            };
            inbox
                .send(Message::QuarantineTimeout { token })
                .await
                .unwrap_or_else(|error| {
                    debug!("Failed to enqueue ZDP quarantine timeout: {error}");
                });
        });
    }
}

fn handle_device_annce(events: &EventSink, device_annce: &DeviceAnnce) {
    let Ok(short_id) = device_annce.nwk_addr().try_into().inspect_err(|error| {
        warn!("Invalid node ID: {error:?}");
    }) else {
        return;
    };

    events.emit(Event::Device(DeviceEvent::Announced(FullAddress::new(
        device_annce.ieee_addr(),
        short_id,
    ))));
}

/// Validate an indication's ZDP addressing and derive its response-correlation key.
fn received_key<T, K>(
    indication: &DataIndication<Frame<Command>, T, K>,
) -> Option<(NetworkAddress, Key)> {
    let source = indication.metadata().source();
    let Some(source_address) = source.network_address() else {
        warn!("Discarding ZDP indication from non-network source: {source:?}");
        return None;
    };
    let Some(key) = Key::from_received_zdp_indication(indication) else {
        warn!(
            "Discarding ZDP indication not addressed between endpoint zero: source={source:?} destination={:?}",
            indication.metadata().destination()
        );
        return None;
    };

    Some((source_address, key))
}

#[cfg(test)]
mod tests;
