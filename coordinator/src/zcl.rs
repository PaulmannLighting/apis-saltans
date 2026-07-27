//! Transceiver to send and receive ZCL messages.

use std::collections::BTreeMap;

use bytes::Bytes;
use le_stream::ToLeStream;
use log::{debug, trace, warn};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot::{self, channel};
use zb_aps::Data;
use zb_core::Destination;
use zb_core::destination::Device;
use zb_nwk::Source;
use zb_zcl::{Cluster, Frame, Header};

pub use self::message::Message;
pub use self::payload::{Metadata, Payload};
pub use self::subscription::{
    Filter as SubscriptionFilter, Received as SubscriptionMessage, Subscription,
    SubscriptionReceiver,
};
use super::index::Index;
use crate::aps::{Aps, TransmissionResponse};
use crate::response::ApsProtocolResponse;
use crate::{Event, MPSC_CHANNEL_SIZE};

mod message;
mod payload;
mod subscription;

/// Zigbee transceiver actor.
#[derive(Debug)]
pub struct Transceiver {
    aps: Aps,
    events: Sender<Event>,
    subscriptions: Vec<Subscription>,
    responses: BTreeMap<Index, oneshot::Sender<Cluster>>,
    seq: u8,
}

impl Transceiver {
    /// Create a new transceiver without frame subscriptions.
    ///
    /// Register subscriptions by sending [`Message::Subscribe`] to the actor.
    #[must_use]
    pub const fn new(aps: Aps, events: Sender<Event>) -> Self {
        Self {
            aps,
            events,
            subscriptions: Vec::new(),
            responses: BTreeMap::new(),
            seq: 0,
        }
    }
    /// Run the transceiver.
    pub async fn run(mut self, mut messages: Receiver<Message>) {
        while let Some(message) = messages.recv().await {
            match message {
                Message::Subscribe { subscription } => {
                    self.subscriptions.push(subscription);
                }
                Message::Received { source, frame } => {
                    self.handle_message_received(source, frame).await;
                }
                Message::Transmit {
                    destination,
                    payload,
                    response,
                } => {
                    response
                        .send(self.transmit(destination, payload).await)
                        .unwrap_or_else(|error| {
                            debug!("Failed to send unicast response: {error:?}");
                        });
                }
                Message::Reply {
                    destination,
                    sequence_number,
                    payload,
                    response,
                } => {
                    response
                        .send(
                            self.transmit_with_sequence(
                                destination.into(),
                                payload,
                                sequence_number,
                            )
                            .await,
                        )
                        .unwrap_or_else(|error| {
                            debug!("Failed to return ZCL reply transmission result: {error:?}");
                        });
                }
                Message::Communicate {
                    device,
                    payload,
                    response,
                } => {
                    response
                        .send(self.communicate(device, payload).await)
                        .unwrap_or_else(|error| {
                            debug!("Failed to send unicast response: {error:?}");
                        });
                }
            }
        }
    }

    /// Return and increment the ZCL sequence number.
    const fn next_seq(&mut self) -> u8 {
        let seq = self.seq;
        self.seq = self.seq.wrapping_add(1);
        seq
    }

    /// Handle a received ZCL message.
    async fn handle_message_received(&mut self, source: Source, aps_frame: Data<Frame<Cluster>>) {
        trace!("Received ZCL message from {source}: {aps_frame:?}");
        if self.forward_to_subscribers(source, &aps_frame).await {
            return;
        }

        let index = Index::from_received_zcl_frame(source, &aps_frame);

        if let Some(sender) = self.responses.remove(&index) {
            let (_, zcl_frame) = aps_frame.into_parts();
            let (_, cluster) = zcl_frame.into_parts();
            sender.send(cluster).unwrap_or_else(|error| {
                debug!("Failed to send ZCL response: {error:?}");
            });

            return;
        }

        let Ok(short_id) = source.node_id().try_into().inspect_err(|error| {
            warn!("Discarding message from invalid source: {source}: {error:?}");
        }) else {
            return;
        };

        self.events
            .send(Event::Zcl {
                src_address: short_id,
                aps_frame,
            })
            .await
            .unwrap_or_else(|error| {
                debug!("Failed to send command: {error:?}");
            });
    }

    /// Deliver a received frame to every matching live subscription.
    async fn forward_to_subscribers(&self, source: Source, frame: &Data<Frame<Cluster>>) -> bool {
        let mut delivered = false;

        for subscription in &self.subscriptions {
            if !subscription.matches(frame) {
                continue;
            }
            let message = SubscriptionMessage {
                source,
                frame: frame.clone(),
            };
            if subscription.send(message).await.is_ok() {
                delivered = true;
            }
        }

        delivered
    }

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
        destination: Destination,
        payload: Payload,
    ) -> Result<TransmissionResponse, zb_hw::Error> {
        let (aps_metadata, zcl_metadata, command) = payload.into_parts();
        let zcl_frame = self.make_zcl_frame(zcl_metadata, command);
        self.aps
            .transmit(
                destination,
                aps_metadata,
                zcl_frame.to_le_stream().collect(),
            )
            .await
    }

    /// Queue a ZCL command with an explicitly selected transaction sequence number.
    async fn transmit_with_sequence(
        &self,
        destination: Destination,
        payload: Payload,
        sequence_number: u8,
    ) -> Result<TransmissionResponse, zb_hw::Error> {
        let (aps_metadata, zcl_metadata, command) = payload.into_parts();
        let zcl_frame = Self::make_zcl_frame_with_sequence(zcl_metadata, command, sequence_number);
        self.aps
            .transmit(
                destination,
                aps_metadata,
                zcl_frame.to_le_stream().collect(),
            )
            .await
    }

    /// Send a ZCL unicast message with back-channel communication.
    ///
    /// # Returns
    ///
    /// Returns the response receiver.
    ///
    /// # Errors
    ///
    /// Returns an error if the unicast message could not be sent.
    async fn communicate(
        &mut self,
        device: Device,
        datagram: Payload,
    ) -> Result<ApsProtocolResponse<Cluster>, zb_hw::Error> {
        let (aps_metadata, zcl_metadata, command) = datagram.into_parts();
        let zcl_frame = self.make_zcl_frame(zcl_metadata, command);
        let index = Index::from_zcl_command(
            device,
            zcl_frame.header().seq(),
            aps_metadata,
            zcl_metadata.manufacturer_code,
        );
        let destination = Destination::from(device);
        let payload = zcl_frame.to_le_stream().collect();
        let (tx, rx) = channel();
        self.responses.insert(index, tx);

        let transmission = match self.aps.transmit(destination, aps_metadata, payload).await {
            Ok(transmission) => transmission,
            Err(error) => {
                self.responses.remove(&index);
                return Err(error);
            }
        };

        Ok(ApsProtocolResponse::new(transmission, rx))
    }

    fn make_zcl_frame(&mut self, metadata: Metadata, command: Bytes) -> Frame<Bytes> {
        Self::make_zcl_frame_with_sequence(metadata, command, self.next_seq())
    }

    fn make_zcl_frame_with_sequence(
        metadata: Metadata,
        command: Bytes,
        sequence_number: u8,
    ) -> Frame<Bytes> {
        Frame::new(
            Header::new(
                metadata.scope,
                metadata.direction,
                metadata.disable_default_response,
                metadata.manufacturer_code,
                sequence_number,
                metadata.command_id,
            ),
            command,
        )
    }
}

impl Transceiver {
    /// Start the ZCL transceiver.
    pub fn spawn(aps: Aps, events: Sender<Event>) -> Sender<Message> {
        let (zcl_tx, zcl_rx) = tokio::sync::mpsc::channel(MPSC_CHANNEL_SIZE);
        spawn(Self::new(aps, events).run(zcl_rx));
        zcl_tx
    }
}

#[cfg(test)]
mod tests {
    use std::future::poll_fn;

    use tokio::runtime::Builder;
    use tokio::sync::mpsc::channel;
    use zb_aps::Data;
    use zb_aps::data::Header as ApsHeader;
    use zb_core::endpoint::Application;
    use zb_core::{Cluster as ClusterId, Direction, Endpoint, Profile};
    use zb_nwk::Source;
    use zb_zcl::on_off::{Command as OnOffCommand, On};
    use zb_zcl::{Cluster, Command, Frame, Header as ZclHeader, Scope};

    use super::{Message, Subscription, SubscriptionFilter, Transceiver};
    use crate::MPSC_CHANNEL_SIZE;
    use crate::aps::Aps;

    const SOURCE_NODE_ID: u16 = 0x4321;
    const TRANSACTION_SEQUENCE: u8 = 7;
    const APS_COUNTER: u8 = 9;

    #[test]
    fn routes_matching_frames_to_a_generic_subscription() {
        Builder::new_current_thread()
            .build()
            .expect("Tokio runtime")
            .block_on(async {
                let (aps_sender, _aps_receiver) = channel(MPSC_CHANNEL_SIZE);
                let (events, mut application_events) = channel(MPSC_CHANNEL_SIZE);
                let filter = SubscriptionFilter::new(
                    ClusterId::OnOff,
                    Scope::ClusterSpecific,
                    Direction::ClientToServer,
                );
                let (subscription, mut subscribed_frames) = Subscription::channel(filter);
                let transceiver = Transceiver::spawn(Aps::new(aps_sender), events);
                let source = Source::new(SOURCE_NODE_ID, None);

                transceiver
                    .send(Message::Subscribe { subscription })
                    .await
                    .expect("ZCL transceiver remains available");
                transceiver
                    .send(Message::Received {
                        source,
                        frame: subscribed_frame(),
                    })
                    .await
                    .expect("ZCL transceiver remains available");

                let received = poll_fn(|context| subscribed_frames.poll_recv(context))
                    .await
                    .expect("subscription remains open");
                assert_eq!(received.source, source);
                assert!(matches!(
                    received.frame.payload().payload(),
                    Cluster::OnOff(OnOffCommand::On(_))
                ));
                assert!(application_events.try_recv().is_err());
            });
    }

    fn subscribed_frame() -> Data<Frame<Cluster>> {
        let endpoint = Endpoint::Application(Application::MIN);
        let aps_header = ApsHeader::new(
            zb_aps::Destination::Unicast(endpoint),
            ClusterId::OnOff.as_u16(),
            Profile::ZigbeeHomeAutomation.as_u16(),
            endpoint,
            APS_COUNTER,
            None,
        );
        let zcl_header = ZclHeader::new(
            Scope::ClusterSpecific,
            Direction::ClientToServer,
            false,
            None,
            TRANSACTION_SEQUENCE,
            <On as Command>::ID,
        );
        Data::new(
            aps_header,
            Frame::new(zcl_header, Cluster::OnOff(OnOffCommand::from(On))),
        )
    }
}
