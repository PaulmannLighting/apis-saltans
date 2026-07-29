//! Transceiver to send and receive ZCL messages.

use std::collections::BTreeMap;

use bytes::Bytes;
use le_stream::ToLeStream;
use log::{debug, trace, warn};
use tokio::spawn;
use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot::{self, channel};
use zb_aps::Data;
use zb_aps::apsde::{DataIndication, DataRequest};
use zb_nwk::Source;
use zb_zcl::{Cluster, Frame, UnsequencedFrame};

pub use self::message::Message;
pub use self::subscription::{
    Filter as SubscriptionFilter, Received as SubscriptionMessage, Subscription,
    SubscriptionReceiver,
};
use super::index::Index;
use crate::aps::{Aps, TransmissionResponse};
use crate::response::ApsProtocolResponse;
use crate::{Error, Event, MPSC_CHANNEL_SIZE};

mod message;
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
    /// Create a ZCL transceiver.
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
                    self.subscriptions.retain(Subscription::is_open);
                    self.subscriptions.push(subscription);
                }
                Message::Unsubscribe { messages } => {
                    self.subscriptions.retain(|subscription| {
                        !subscription.same_channel(&messages) && subscription.is_open()
                    });
                }
                Message::Received { indication } => {
                    self.handle_message_received(indication).await;
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
                Message::Communicate { request, response } => {
                    response
                        .send(self.communicate(request).await)
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
    async fn handle_message_received(
        &mut self,
        indication: DataIndication<Frame<Cluster>, (), ()>,
    ) {
        let Some((source, aps_frame)) = crate::apsde::into_legacy_data(indication) else {
            warn!("Discarding ZCL indication with unsupported addressing");
            return;
        };
        trace!("Received ZCL message from {source}: {aps_frame:?}");
        if self.forward_to_subscribers(source, &aps_frame) {
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
    fn forward_to_subscribers(&mut self, source: Source, frame: &Data<Frame<Cluster>>) -> bool {
        let mut delivered = false;

        self.subscriptions.retain(|subscription| {
            if !subscription.is_open() {
                return false;
            }
            if !subscription.matches(frame) {
                return true;
            }
            let message = SubscriptionMessage {
                source,
                frame: frame.clone(),
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
        let sequence_number = self.next_seq();
        let request = Self::encode_request(request, sequence_number);
        Ok(self.aps.transmit(request).await?)
    }

    /// Queue a ZCL command with an explicitly selected transaction sequence number.
    async fn transmit_with_sequence(
        &self,
        request: DataRequest<UnsequencedFrame<Bytes>>,
        sequence_number: u8,
    ) -> Result<TransmissionResponse, Error> {
        let request = Self::encode_request(request, sequence_number);
        Ok(self.aps.transmit(request).await?)
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
        request: DataRequest<UnsequencedFrame<Bytes>>,
    ) -> Result<ApsProtocolResponse<Cluster>, Error> {
        let sequence_number = self.next_seq();
        let index = Self::request_index(&request, sequence_number)?;
        let request = Self::encode_request(request, sequence_number);
        let (tx, rx) = channel();
        self.responses.insert(index, tx);

        let transmission = match self.aps.transmit(request).await {
            Ok(transmission) => transmission,
            Err(error) => {
                self.responses.remove(&index);
                return Err(error.into());
            }
        };

        Ok(ApsProtocolResponse::new(transmission, rx))
    }

    fn encode_request(
        request: DataRequest<UnsequencedFrame<Bytes>>,
        sequence_number: u8,
    ) -> DataRequest<Bytes> {
        request.map_asdu(|frame| frame.into_frame(sequence_number).to_le_stream().collect())
    }

    const fn request_index(
        request: &DataRequest<UnsequencedFrame<Bytes>>,
        sequence_number: u8,
    ) -> Result<Index, Error> {
        let zb_aps::apsde::RequestDestination::Network { address, endpoint } =
            request.destination()
        else {
            return Err(Error::InvalidZclCommunicationDestination(
                request.destination(),
            ));
        };
        if zb_aps::apsde::IndividualEndpoint::new(endpoint).is_none() {
            return Err(Error::InvalidZclCommunicationDestination(
                request.destination(),
            ));
        }

        Ok(Index::new(
            address.as_u16(),
            endpoint,
            request.cluster_id(),
            request.profile_id(),
            request.asdu().header().manufacturer_code(),
            sequence_number,
        ))
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
    use tokio::runtime::Builder;
    use tokio::sync::mpsc::channel;
    use zb_aps::apsde::{
        Alias, DataIndication, DataRequest, IndicationMetadata, IndicationStatus,
        IndividualEndpoint, NetworkAddress, ReceivedDestination, RequestDestination, Security,
        Source as ApsdeSource,
    };
    use zb_aps::data::Header as ApsHeader;
    use zb_aps::{Data, TxOptions};
    use zb_core::endpoint::Application;
    use zb_core::{Cluster as ClusterId, Direction, Endpoint, Profile};
    use zb_nwk::Source;
    use zb_zcl::on_off::{Command as OnOffCommand, On};
    use zb_zcl::{Cluster, Command, Frame, Header as ZclHeader, Scope, UnsequencedFrame};

    use super::{Message, Subscription, SubscriptionFilter, SubscriptionMessage, Transceiver};
    use crate::aps::Aps;
    use crate::{Error, Event, MPSC_CHANNEL_SIZE};

    const SOURCE_NODE_ID: u16 = 0x4321;
    const TRANSACTION_SEQUENCE: u8 = 7;
    const APS_COUNTER: u8 = 9;
    const LINK_QUALITY: u8 = 255;
    const LOCAL_NODE_ID: u16 = 0;
    const LOCAL_ENDPOINT_ID: u8 = 11;
    const REMOTE_ENDPOINT_ID: u8 = 12;
    const RADIUS_COUNTER: u8 = 5;
    const ALIAS_SEQUENCE_NUMBER: u8 = 6;

    #[test]
    fn encoding_preserves_every_aps_request_field() {
        let destination_address =
            NetworkAddress::new(SOURCE_NODE_ID).expect("test NWK address is valid");
        let alias_address =
            NetworkAddress::new(APS_COUNTER.into()).expect("test alias address is valid");
        let destination = RequestDestination::Network {
            address: destination_address,
            endpoint: Endpoint::from(REMOTE_ENDPOINT_ID),
        };
        let source_endpoint = IndividualEndpoint::new(Endpoint::from(LOCAL_ENDPOINT_ID))
            .expect("application endpoint is individual");
        let tx_options = TxOptions::SECURITY_ENABLED | TxOptions::ACKNOWLEDGED_TRANSMISSION;
        let alias = Alias::Use {
            source: alias_address,
            sequence_number: ALIAS_SEQUENCE_NUMBER,
        };
        let request = DataRequest::new(
            destination,
            Profile::ZigbeeHomeAutomation.as_u16(),
            ClusterId::OnOff.as_u16(),
            source_endpoint,
            UnsequencedFrame::from_command(On),
        )
        .with_tx_options(tx_options)
        .with_alias(alias)
        .with_radius_counter(RADIUS_COUNTER);

        let encoded = Transceiver::encode_request(request, TRANSACTION_SEQUENCE);
        let frame = Frame::parse(
            ClusterId::OnOff.as_u16(),
            encoded.asdu().clone().into_iter(),
        )
        .expect("encoded command is a valid ZCL frame");

        assert_eq!(encoded.destination(), destination);
        assert_eq!(encoded.profile_id(), Profile::ZigbeeHomeAutomation.as_u16());
        assert_eq!(encoded.cluster_id(), ClusterId::OnOff.as_u16());
        assert_eq!(encoded.source_endpoint(), source_endpoint);
        assert_eq!(encoded.tx_options(), tx_options);
        assert_eq!(encoded.alias(), alias);
        assert_eq!(encoded.radius_counter(), RADIUS_COUNTER);
        assert_eq!(frame.header().seq(), TRANSACTION_SEQUENCE);
        assert!(matches!(
            frame.payload(),
            Cluster::OnOff(OnOffCommand::On(_))
        ));
    }

    #[test]
    fn communication_rejects_a_non_network_destination() {
        let source_endpoint = IndividualEndpoint::new(Endpoint::from(LOCAL_ENDPOINT_ID))
            .expect("application endpoint is individual");
        let request = DataRequest::new(
            RequestDestination::Bound,
            Profile::ZigbeeHomeAutomation.as_u16(),
            ClusterId::OnOff.as_u16(),
            source_endpoint,
            UnsequencedFrame::from_command(On),
        );

        assert!(matches!(
            Transceiver::request_index(&request, TRANSACTION_SEQUENCE),
            Err(Error::InvalidZclCommunicationDestination(
                RequestDestination::Bound
            ))
        ));
    }

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
                let (transceiver, messages) = channel(MPSC_CHANNEL_SIZE);
                tokio::spawn(Transceiver::new(Aps::new(aps_sender), events).run(messages));
                let source = Source::new(SOURCE_NODE_ID, None);

                transceiver
                    .send(Message::Subscribe { subscription })
                    .await
                    .expect("ZCL transceiver remains available");
                transceiver
                    .send(Message::Received {
                        indication: subscribed_indication(),
                    })
                    .await
                    .expect("ZCL transceiver remains available");

                let received = subscribed_frames
                    .recv()
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

    #[test]
    fn unregisters_a_subscription() {
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
                let (subscription, subscribed_frames) = Subscription::channel(filter);
                let subscription_messages = subscribed_frames.sender();
                let (transceiver, messages) = channel(MPSC_CHANNEL_SIZE);
                tokio::spawn(Transceiver::new(Aps::new(aps_sender), events).run(messages));

                transceiver
                    .send(Message::Subscribe { subscription })
                    .await
                    .expect("ZCL transceiver remains available");
                transceiver
                    .send(Message::Unsubscribe {
                        messages: subscription_messages,
                    })
                    .await
                    .expect("ZCL transceiver remains available");
                transceiver
                    .send(Message::Received {
                        indication: subscribed_indication(),
                    })
                    .await
                    .expect("ZCL transceiver remains available");

                assert!(matches!(
                    application_events.recv().await,
                    Some(Event::Zcl { .. })
                ));
            });
    }

    #[test]
    fn removes_closed_subscriptions_during_delivery() {
        let (subscription, receiver) = Subscription::channel(SubscriptionFilter::new(
            ClusterId::OnOff,
            Scope::ClusterSpecific,
            Direction::ClientToServer,
        ));
        let (mut transceiver, _events) = unstarted_transceiver();
        transceiver.subscriptions.push(subscription);
        drop(receiver);

        let delivered = transceiver
            .forward_to_subscribers(Source::new(SOURCE_NODE_ID, None), &subscribed_frame());

        assert!(!delivered);
        assert!(transceiver.subscriptions.is_empty());
    }

    #[test]
    fn full_subscription_does_not_block_normal_routing() {
        Builder::new_current_thread()
            .build()
            .expect("Tokio runtime")
            .block_on(async {
                let (subscription, _receiver) = Subscription::channel(SubscriptionFilter::new(
                    ClusterId::OnOff,
                    Scope::ClusterSpecific,
                    Direction::ClientToServer,
                ));
                let source = Source::new(SOURCE_NODE_ID, None);
                for _ in 0..MPSC_CHANNEL_SIZE {
                    subscription
                        .try_send(SubscriptionMessage {
                            source,
                            frame: subscribed_frame(),
                        })
                        .expect("subscription channel has capacity");
                }
                let (mut transceiver, mut events) = unstarted_transceiver();
                transceiver.subscriptions.push(subscription);

                transceiver
                    .handle_message_received(subscribed_indication())
                    .await;

                assert!(matches!(events.try_recv(), Ok(Event::Zcl { .. })));
                assert_eq!(transceiver.subscriptions.len(), 1);
            });
    }

    fn unstarted_transceiver() -> (Transceiver, tokio::sync::mpsc::Receiver<Event>) {
        let (aps_sender, _aps_receiver) = channel(MPSC_CHANNEL_SIZE);
        let (events, application_events) = channel(MPSC_CHANNEL_SIZE);
        (
            Transceiver::new(Aps::new(aps_sender), events),
            application_events,
        )
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

    fn subscribed_indication() -> DataIndication<Frame<Cluster>, (), ()> {
        let endpoint = IndividualEndpoint::new(Endpoint::Application(Application::MIN))
            .expect("application endpoint is individual");
        let metadata = IndicationMetadata::new(
            ReceivedDestination::Network {
                address: NetworkAddress::new(LOCAL_NODE_ID)
                    .expect("coordinator address is a valid NWK address"),
                endpoint,
            },
            ApsdeSource::Network {
                address: NetworkAddress::new(SOURCE_NODE_ID)
                    .expect("source address is a valid NWK address"),
                endpoint,
            },
            Profile::ZigbeeHomeAutomation.as_u16(),
            ClusterId::OnOff.as_u16(),
            IndicationStatus::success(),
            Security::Unsecured,
            LINK_QUALITY,
            (),
        );
        let (_, frame) = subscribed_frame().into_parts();
        DataIndication::new(metadata, frame)
    }
}
