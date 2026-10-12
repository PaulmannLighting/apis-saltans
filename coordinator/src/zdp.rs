//! Transceiver to send and receive ZDP messages.

use std::time::Duration;

use bytes::Bytes;
use le_stream::ToLeStream;
use log::{debug, error, trace, warn};
use tokio::sync::mpsc::{Receiver, Sender, WeakSender};
use zb_aps::apsde::{DataIndication, DataRequest, NetworkAddress, ReceivedDestination};
use zb_core::FullAddress;
use zb_core::node::Descriptor;
use zb_core::short_id::Device;
use zb_hw::NcpHandle;
use zb_zdp::{Command, DeviceAndServiceDiscovery, DeviceAnnce, Frame};

pub use self::key::ZdpKey;
pub use self::message::Message;
use self::server::{Server, ServerRequest, is_server_request};
use self::server_operations::ServerOperations;
use self::submission::Submissions;
use crate::aps::Aps;
use crate::correlation::{
    PROTOCOL_QUARANTINE_TIMEOUT, PROTOCOL_RESPONSE_TIMEOUT, Registry, Token, schedule_timeout,
};
use crate::event::EventSink;
use crate::response::ApsProtocolResponse;
use crate::{CoordinatorConfig, Device as DeviceEvent, Event, MPSC_CHANNEL_SIZE};

mod discovery;
mod key;
mod match_desc;
mod message;
mod node_desc;
mod server;
mod server_operations;
mod submission;

#[cfg(test)]
mod tests;

/// Zigbee transceiver actor.
#[derive(Debug)]
pub struct Transceiver {
    server: Server,
    events: EventSink,
    responses: Registry<ZdpKey, Command>,
    inbox: WeakSender<Message>,
    submissions: Submissions,
    server_operations: ServerOperations,
    response_timeout: Duration,
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
            server: Server::new(ncp, aps.clone(), descriptor, inbox.clone()),
            events,
            responses: Registry::new(),
            submissions: Submissions::new(aps, inbox.clone(), MPSC_CHANNEL_SIZE),
            inbox,
            server_operations: ServerOperations::new(MPSC_CHANNEL_SIZE),
            response_timeout: PROTOCOL_RESPONSE_TIMEOUT,
        }
    }

    /// Start the ZDP transceiver.
    pub fn spawn(
        ncp: NcpHandle,
        aps: Aps,
        events: EventSink,
        descriptor: Descriptor,
        config: &CoordinatorConfig,
    ) -> Sender<Message> {
        let (zdp_tx, zdp_rx) = tokio::sync::mpsc::channel(config.inbox_capacity.get());
        let mut actor = Self::new(ncp, aps, events, descriptor, zdp_tx.downgrade());
        actor.response_timeout = config.protocol_response_timeout;
        actor.submissions.set_limit(config.zdp_submission_limit);
        actor
            .server_operations
            .set_limit(config.zdp_server_operation_limit);
        crate::runtime::spawn_actor("ZDP", actor.run(zdp_rx));
        zdp_tx
    }

    /// Run the transceiver.
    pub async fn run(mut self, mut messages: Receiver<Message>) {
        while let Some(message) = messages.recv().await {
            if !self.handle_actor_message(message) {
                break;
            }
        }
        self.server_operations.abort();
        self.submissions.abort();
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
                self.server_operations.abort();
                self.handle_network_down();
            }
            Message::HardwareUnavailable => {
                self.server_operations.abort();
                self.submissions.hardware_unavailable();
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
                self.server_operations.finish(id);
            }
            Message::CommunicationSubmissionFinished { id, result } => {
                self.submissions.finish(id, result, &mut self.responses);
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
                self.server_operations.spawn(
                    &self.server,
                    self.inbox.clone(),
                    ServerRequest::new(source_address, request_was_broadcast, seq, command),
                );
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
        if !self.submissions.has_capacity() {
            response
                .send(Err(crate::Error::SendError))
                .unwrap_or_else(drop);
            return;
        }
        let (seq, token, protocol_response) = match self
            .responses
            .register(|sequence| ZdpKey::from_command(device, sequence, &request))
        {
            Ok(registration) => registration,
            Err(error) => {
                response.send(Err(error)).unwrap_or_else(drop);
                return;
            }
        };
        self.schedule_response_timeout(token);
        let request = request.map_asdu(|payload| Frame::new(seq, payload).to_le_stream().collect());
        self.submissions
            .start(request, token, protocol_response, response);
    }

    /// Reset response correlation while preserving identities exposed by in-flight handoffs.
    fn handle_network_down(&mut self) {
        for token in self.submissions.network_down(&mut self.responses) {
            self.schedule_quarantine_timeout(token);
        }
    }
}

/// Pending-response cancellation, timeout, and quarantine lifecycle management.
impl Transceiver {
    fn schedule_response_timeout(&self, token: Token<ZdpKey>) {
        schedule_timeout(
            self.inbox.clone(),
            self.response_timeout,
            Message::ResponseTimeout { token },
            "ZDP response timeout",
        );
    }

    fn schedule_quarantine_timeout(&self, token: Token<ZdpKey>) {
        schedule_timeout(
            self.inbox.clone(),
            PROTOCOL_QUARANTINE_TIMEOUT,
            Message::QuarantineTimeout { token },
            "ZDP quarantine timeout",
        );
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
) -> Option<(NetworkAddress, ZdpKey)> {
    let source = indication.metadata().source();
    let Some(source_address) = source.network_address() else {
        warn!("Discarding ZDP indication from non-network source: {source:?}");
        return None;
    };
    let Some(key) = ZdpKey::from_indication(indication) else {
        warn!(
            "Discarding ZDP indication not addressed between endpoint zero: source={source:?} destination={:?}",
            indication.metadata().destination()
        );
        return None;
    };

    Some((source_address, key))
}

impl Drop for Transceiver {
    fn drop(&mut self) {
        self.server_operations.abort();
        self.submissions.hardware_unavailable();
        self.responses.hardware_unavailable();
    }
}
