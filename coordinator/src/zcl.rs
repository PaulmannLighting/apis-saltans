//! Transceiver to send and receive ZCL messages.

use bytes::Bytes;
use le_stream::ToLeStream;
use log::{debug, trace, warn};
use tokio::runtime::Handle;
use tokio::spawn;
use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::mpsc::{Receiver, Sender, WeakSender};
use tokio::time::sleep;
use zb_aps::Data;
use zb_aps::apsde::{DataIndication, DataRequest, Source};
use zb_core::Direction;
use zb_zcl::{Cluster, Frame, UnsequencedFrame};

pub use self::message::Message;
pub use self::subscription::{
    Filter as SubscriptionFilter, Received as SubscriptionMessage, Subscription,
    SubscriptionReceiver,
};
use super::index::Index;
use crate::aps::{Aps, TransmissionResponse};
use crate::correlation::{Cancellation, PROTOCOL_RESPONSE_TIMEOUT, Registry, Token};
use crate::event_sink::EventSink;
use crate::response::ApsProtocolResponse;
use crate::{Error, Event, MPSC_CHANNEL_SIZE};

mod message;
mod subscription;

/// Zigbee transceiver actor.
#[derive(Debug)]
pub struct Transceiver {
    aps: Aps,
    events: EventSink,
    subscriptions: Vec<Subscription>,
    responses: Registry<Cluster>,
    inbox: WeakSender<Message>,
}

impl Transceiver {
    /// Create a ZCL transceiver.
    pub const fn new(aps: Aps, events: EventSink, inbox: WeakSender<Message>) -> Self {
        Self {
            aps,
            events,
            subscriptions: Vec::new(),
            responses: Registry::new(),
            inbox,
        }
    }

    /// Run the transceiver.
    pub async fn run(mut self, mut messages: Receiver<Message>) {
        while let Some(message) = messages.recv().await {
            self.handle_actor_message(message).await;
        }
    }

    async fn handle_actor_message(&mut self, message: Message) {
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
                self.responses
                    .network_down(&zb_hw::TransmissionError::NoRoute);
            }
            Message::Cancel { token } => {
                self.responses.cancel(token);
            }
            Message::ResponseTimeout { token } => {
                self.responses.timeout(token);
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

    /// Handle a received ZCL message.
    fn handle_message_received(&mut self, indication: DataIndication<Frame<Cluster>, (), ()>) {
        let Some((source, header)) = crate::apsde::legacy_context(indication.metadata()) else {
            warn!("Discarding ZCL indication with unsupported addressing");
            return;
        };
        let Some(source_address) = source.network_address() else {
            warn!("Discarding ZCL indication from non-network source: {source:?}");
            return;
        };
        let aps_frame = Data::new(header, indication.asdu().clone());
        trace!("Received ZCL message from {source:?}: {aps_frame:?}");
        let index = Index::from_received_zcl_frame(source_address, &aps_frame);

        let (_, zcl_frame) = aps_frame.clone().into_parts();
        let (_, cluster) = zcl_frame.into_parts();
        if self.responses.complete(index, cluster) {
            return;
        }
        if self.responses.release_quarantine(index) {
            debug!(
                "Discarding late ZCL response with quarantined sequence {}",
                index.sequence()
            );
            return;
        }
        if self.forward_to_subscribers(source, &aps_frame) {
            return;
        }

        self.events.emit(Event::Zcl { indication });
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
        let sequence_number = self
            .responses
            .reserve_untracked_sequence(|sequence| Self::request_index(&request, sequence).ok())?;
        let response_index = Self::request_index(&request, sequence_number).ok();
        let request = Self::encode_request(request, sequence_number);
        match self.aps.transmit(request).await {
            Ok(transmission) => Ok(transmission),
            Err(error) => {
                if let Some(index) = response_index {
                    self.responses.release_quarantine(index);
                }
                Err(error)
            }
        }
    }

    /// Queue a ZCL command with an explicitly selected transaction sequence number.
    async fn transmit_with_sequence(
        &self,
        request: DataRequest<UnsequencedFrame<Bytes>>,
        sequence_number: u8,
    ) -> Result<TransmissionResponse, Error> {
        let request = Self::encode_request(request, sequence_number);
        self.aps.transmit(request).await
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
        Self::request_index(&request, u8::MIN)?;
        let (sequence_number, token, rx) = self.responses.register(|sequence| {
            Self::request_index(&request, sequence)
                .expect("ZCL communication destination was validated")
        })?;
        self.schedule_response_timeout(token);
        let request = Self::encode_request(request, sequence_number);

        let transmission = match self.aps.transmit(request).await {
            Ok(transmission) => transmission,
            Err(error) => {
                self.responses.discard(token);
                return Err(error);
            }
        };
        let cancellation = self.cancellation(token);

        Ok(ApsProtocolResponse::new(transmission, rx, cancellation))
    }

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
                            debug!("Failed to enqueue ZCL response cancellation: {error}");
                        });
                    });
                }
                Err(TrySendError::Closed(_)) => {
                    debug!("Failed to enqueue ZCL response cancellation: actor unavailable");
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
                    debug!("Failed to enqueue ZCL response timeout: {error}");
                });
        });
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

        Ok(Index::new_zcl(
            address.as_u16(),
            endpoint,
            request.cluster_id(),
            request.profile_id(),
            request.asdu().header().manufacturer_code(),
            response_direction(request.asdu().header().control().direction()),
            sequence_number,
        ))
    }
}

const fn response_direction(request_direction: Direction) -> Direction {
    match request_direction {
        Direction::ClientToServer => Direction::ServerToClient,
        Direction::ServerToClient => Direction::ClientToServer,
    }
}

impl Transceiver {
    /// Start the ZCL transceiver.
    pub fn spawn(aps: Aps, events: EventSink) -> Sender<Message> {
        let (zcl_tx, zcl_rx) = tokio::sync::mpsc::channel(MPSC_CHANNEL_SIZE);
        spawn(Self::new(aps, events, zcl_tx.downgrade()).run(zcl_rx));
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
        Source,
    };
    use zb_aps::data::Header as ApsHeader;
    use zb_aps::{Data, TxOptions};
    use zb_core::endpoint::Application;
    use zb_core::{Cluster as ClusterId, Direction, Endpoint, Profile};
    use zb_zcl::on_off::{Command as OnOffCommand, On};
    use zb_zcl::{Cluster, Command, Frame, Header as ZclHeader, Scope, UnsequencedFrame};

    use super::{Message, Subscription, SubscriptionFilter, SubscriptionMessage, Transceiver};
    use crate::aps::Aps;
    use crate::event_sink::EventSink;
    use crate::index::Index;
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
                tokio::spawn(
                    Transceiver::new(
                        Aps::new(aps_sender),
                        EventSink::new(events),
                        transceiver.downgrade(),
                    )
                    .run(messages),
                );
                let source = source();

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
    fn matching_subscription_does_not_consume_correlated_response() {
        Builder::new_current_thread()
            .build()
            .expect("Tokio runtime")
            .block_on(async {
                let filter = SubscriptionFilter::new(
                    ClusterId::OnOff,
                    Scope::ClusterSpecific,
                    Direction::ClientToServer,
                );
                let (subscription, mut subscribed_frames) = Subscription::channel(filter);
                let (mut transceiver, mut events) = unstarted_transceiver();
                transceiver.subscriptions.push(subscription);
                let frame = subscribed_frame();
                let source_address =
                    NetworkAddress::new(SOURCE_NODE_ID).expect("source address is valid");
                let response_index = Index::from_received_zcl_frame(source_address, &frame);
                let (_, _, response) = transceiver
                    .responses
                    .register(|_| response_index)
                    .expect("response correlation can be registered");

                transceiver.handle_message_received(subscribed_indication());

                assert!(matches!(
                    response.await,
                    Ok(Ok(Cluster::OnOff(OnOffCommand::On(_))))
                ));
                assert!(subscribed_frames.try_recv().is_err());
                assert!(events.try_recv().is_err());
            });
    }

    #[test]
    fn subscription_request_does_not_complete_opposite_direction_response() {
        Builder::new_current_thread()
            .build()
            .expect("Tokio runtime")
            .block_on(async {
                let filter = SubscriptionFilter::new(
                    ClusterId::OnOff,
                    Scope::ClusterSpecific,
                    Direction::ClientToServer,
                );
                let (subscription, mut subscribed_frames) = Subscription::channel(filter);
                let (mut transceiver, _events) = unstarted_transceiver();
                transceiver.subscriptions.push(subscription);
                let endpoint = Endpoint::Application(Application::MIN);
                let response_index = Index::new_zcl(
                    SOURCE_NODE_ID,
                    endpoint,
                    ClusterId::OnOff.as_u16(),
                    Profile::ZigbeeHomeAutomation.as_u16(),
                    None,
                    Direction::ServerToClient,
                    TRANSACTION_SEQUENCE,
                );
                let (_, _, mut response) = transceiver
                    .responses
                    .register(|_| response_index)
                    .expect("response correlation can be registered");

                transceiver.handle_message_received(subscribed_indication());

                assert!(subscribed_frames.recv().await.is_some());
                assert!(response.try_recv().is_err());
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
                tokio::spawn(
                    Transceiver::new(
                        Aps::new(aps_sender),
                        EventSink::new(events),
                        transceiver.downgrade(),
                    )
                    .run(messages),
                );

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

                let Some(Event::Zcl { indication }) = application_events.recv().await else {
                    panic!("expected unmatched ZCL indication");
                };
                assert_eq!(indication.metadata().source(), source());
                assert_eq!(indication.metadata().link_quality(), LINK_QUALITY);
                assert!(matches!(
                    indication.asdu().payload(),
                    Cluster::OnOff(OnOffCommand::On(_))
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

        let delivered = transceiver.forward_to_subscribers(source(), &subscribed_frame());

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
                let source = source();
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

                transceiver.handle_message_received(subscribed_indication());

                assert!(matches!(events.try_recv(), Ok(Event::Zcl { .. })));
                assert_eq!(transceiver.subscriptions.len(), 1);
            });
    }

    fn unstarted_transceiver() -> (Transceiver, tokio::sync::mpsc::Receiver<Event>) {
        let (aps_sender, _aps_receiver) = channel(MPSC_CHANNEL_SIZE);
        let (events, application_events) = channel(MPSC_CHANNEL_SIZE);
        let (inbox, _messages) = channel(MPSC_CHANNEL_SIZE);
        (
            Transceiver::new(
                Aps::new(aps_sender),
                EventSink::new(events),
                inbox.downgrade(),
            ),
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
            Source::Network {
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

    fn source() -> Source {
        let endpoint = IndividualEndpoint::new(Endpoint::Application(Application::MIN))
            .expect("application endpoint is individual");
        Source::Network {
            address: NetworkAddress::new(SOURCE_NODE_ID)
                .expect("source address is a valid NWK address"),
            endpoint,
        }
    }
}
