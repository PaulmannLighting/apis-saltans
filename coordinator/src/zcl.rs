//! Transceiver to send and receive ZCL messages.

use bytes::Bytes;
use le_stream::ToLeStream;
use log::{debug, trace, warn};
use tokio::spawn;
use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::mpsc::{Receiver, Sender, WeakSender};
use zb_aps::apsde::{DataIndication, DataRequest};
use zb_zcl::{Cluster, Frame, UnsequencedFrame};

pub use self::correlation::ResponseExpectation;
#[cfg(test)]
pub(crate) use self::correlation::ResponseMatch;
use self::correlation::Responses;
pub use self::message::Message;
pub use self::subscription::{
    Filter as SubscriptionFilter, Received as SubscriptionMessage, Subscription,
    SubscriptionReceiver,
};
use crate::aps::{Aps, TransmissionResponse};
use crate::event::EventSink;
use crate::{Error, Event, MPSC_CHANNEL_SIZE};

mod correlation;
mod message;
mod subscription;

/// Zigbee transceiver actor.
#[derive(Debug)]
pub struct Transceiver {
    aps: Aps,
    events: EventSink,
    subscriptions: Vec<Subscription>,
    responses: Responses,
}

/// Construction, startup, and actor-inbox processing.
impl Transceiver {
    /// Create a ZCL transceiver.
    pub const fn new(aps: Aps, events: EventSink, inbox: WeakSender<Message>) -> Self {
        Self {
            aps,
            events,
            subscriptions: Vec::new(),
            responses: Responses::new(inbox),
        }
    }

    /// Start the ZCL transceiver.
    pub fn spawn(aps: Aps, events: EventSink) -> Sender<Message> {
        let (zcl_tx, zcl_rx) = tokio::sync::mpsc::channel(MPSC_CHANNEL_SIZE);
        spawn(Self::new(aps, events, zcl_tx.downgrade()).run(zcl_rx));
        zcl_tx
    }

    /// Run the transceiver.
    pub async fn run(mut self, mut messages: Receiver<Message>) {
        while let Some(message) = messages.recv().await {
            if !self.handle_actor_message(message).await {
                break;
            }
        }
    }

    async fn handle_actor_message(&mut self, message: Message) -> bool {
        match message {
            Message::Subscribe { subscription } => {
                self.subscriptions.retain(Subscription::is_open);
                self.subscriptions.push(subscription);
            }
            Message::Unsubscribe { messages } => {
                self.subscriptions.retain(|subscription| {
                    !subscription.same_channel(&messages) && subscription.is_open()
                });
            }
            Message::Received { indication } => {
                self.handle_message_received(indication);
            }
            Message::NetworkDown => {
                self.responses.network_down();
            }
            Message::HardwareUnavailable => {
                self.responses.hardware_unavailable();
                return false;
            }
            Message::Cancel { token } => {
                self.responses.cancel(token);
            }
            Message::ResponseTimeout { token } => {
                self.responses.timeout(token);
            }
            Message::QuarantineTimeout { token } => {
                self.responses.expire_quarantine(token);
            }
            Message::Transmit { request, response } => {
                response
                    .send(self.transmit(request).await)
                    .unwrap_or_else(|error| {
                        debug!("Failed to send unicast response: {error:?}");
                    });
            }
            Message::Reply {
                sequence_number,
                request,
                response,
            } => {
                response
                    .send(self.transmit_with_sequence(request, sequence_number).await)
                    .unwrap_or_else(|error| {
                        debug!("Failed to return ZCL reply transmission result: {error:?}");
                    });
            }
            Message::Communicate {
                request,
                expected,
                response,
            } => {
                response
                    .send(
                        self.responses
                            .communicate(&self.aps, request, expected)
                            .await,
                    )
                    .unwrap_or_else(|error| {
                        debug!("Failed to send unicast response: {error:?}");
                    });
            }
        }
        true
    }
}

/// Inbound response correlation, subscription delivery, and application-event routing.
impl Transceiver {
    /// Handle a received ZCL message.
    fn handle_message_received(&mut self, indication: DataIndication<Frame<Cluster>, (), ()>) {
        let source = indication.metadata().source();
        trace!("Received ZCL message from {source:?}: {indication:?}");

        if self.responses.handle_received(&indication) {
            return;
        }
        if self.forward_to_subscribers(&indication) {
            return;
        }

        self.events.emit(Event::Zcl { indication });
    }

    /// Deliver a received frame to every matching live subscription.
    fn forward_to_subscribers(
        &mut self,
        indication: &DataIndication<Frame<Cluster>, (), ()>,
    ) -> bool {
        let mut delivered = false;

        self.subscriptions.retain(|subscription| {
            if !subscription.is_open() {
                return false;
            }

            if !subscription.matches(indication) {
                return true;
            }

            let message = SubscriptionMessage {
                indication: indication.clone(),
            };

            match subscription.try_send(message) {
                Ok(()) => {
                    delivered = true;
                    true
                }
                Err(TrySendError::Full(_)) => {
                    warn!("ZCL subscription channel is full; forwarding frame to normal routing");
                    true
                }
                Err(TrySendError::Closed(_)) => false,
            }
        });

        delivered
    }
}

/// Outbound ZCL transmission and request-response communication.
impl Transceiver {
    /// Queue a ZCL message and return its deferred APS transmission result.
    ///
    /// # Returns
    ///
    /// Returns the deferred APS transmission response.
    ///
    /// # Errors
    ///
    /// Returns an error if the unicast message could not be sent.
    async fn transmit(
        &mut self,
        request: DataRequest<UnsequencedFrame<Bytes>>,
    ) -> Result<TransmissionResponse, Error> {
        let sequence_number = self.responses.allocate_untracked(&request)?;

        self.aps
            .transmit(Self::encode_request(request, sequence_number))
            .await
    }

    /// Queue a ZCL command with an explicitly selected transaction sequence number.
    async fn transmit_with_sequence(
        &self,
        request: DataRequest<UnsequencedFrame<Bytes>>,
        sequence_number: u8,
    ) -> Result<TransmissionResponse, Error> {
        self.aps
            .transmit(Self::encode_request(request, sequence_number))
            .await
    }
}

/// Outbound frame encoding.
impl Transceiver {
    fn encode_request(
        request: DataRequest<UnsequencedFrame<Bytes>>,
        sequence_number: u8,
    ) -> DataRequest<Bytes> {
        request.map_asdu(|frame| frame.into_frame(sequence_number).to_le_stream().collect())
    }
}

#[cfg(test)]
mod tests;
