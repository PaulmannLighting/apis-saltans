//! Transceiver to send and receive ZCL messages.

use std::collections::BTreeMap;

use bytes::Bytes;
use le_stream::ToLeStream;
use log::{debug, trace, warn};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot::{self, channel};
use zb_aps::Data;
use zb_core::destination::Device;
use zb_core::{Destination, Direction, Endpoint};
use zb_hw::NcpHandle;
use zb_nwk::Source;
use zb_zcl::{Cluster, Frame, Header};
use zb_zdp::SimpleDescriptor;

pub use self::message::Message;
pub use self::payload::{Metadata, Payload};
pub use self::subscription::{
    Filter as SubscriptionFilter, Received as SubscriptionMessage, Subscription,
    SubscriptionReceiver,
};
use super::index::Index;
use crate::aps::{Aps, TransmissionResponse};
use crate::response::ApsProtocolResponse;
use crate::{Error, Event, MPSC_CHANNEL_SIZE};

mod message;
mod payload;
mod subscription;

/// Zigbee transceiver actor.
#[derive(Debug)]
pub struct Transceiver {
    ncp: NcpHandle,
    aps: Aps,
    events: Sender<Event>,
    endpoints: Option<Box<[SimpleDescriptor]>>,
    subscriptions: Vec<Subscription>,
    responses: BTreeMap<Index, oneshot::Sender<Cluster>>,
    seq: u8,
}

impl Transceiver {
    /// Create a transceiver that obtains local endpoint descriptors from the NCP on demand.
    pub const fn new(ncp: NcpHandle, aps: Aps, events: Sender<Event>) -> Self {
        Self {
            ncp,
            aps,
            events,
            endpoints: None,
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
    ) -> Result<TransmissionResponse, Error> {
        let (aps_metadata, zcl_metadata, command) = payload.into_parts();
        let source_endpoint = self
            .source_endpoint(aps_metadata, zcl_metadata.direction)
            .await?;
        let zcl_frame = self.make_zcl_frame(zcl_metadata, command);
        Ok(self
            .aps
            .transmit(
                destination,
                source_endpoint,
                aps_metadata,
                zcl_frame.to_le_stream().collect(),
            )
            .await?)
    }

    /// Queue a ZCL command with an explicitly selected transaction sequence number.
    async fn transmit_with_sequence(
        &mut self,
        destination: Destination,
        payload: Payload,
        sequence_number: u8,
    ) -> Result<TransmissionResponse, Error> {
        let (aps_metadata, zcl_metadata, command) = payload.into_parts();
        let source_endpoint = self
            .source_endpoint(aps_metadata, zcl_metadata.direction)
            .await?;
        let zcl_frame = Self::make_zcl_frame_with_sequence(zcl_metadata, command, sequence_number);
        Ok(self
            .aps
            .transmit(
                destination,
                source_endpoint,
                aps_metadata,
                zcl_frame.to_le_stream().collect(),
            )
            .await?)
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
    ) -> Result<ApsProtocolResponse<Cluster>, Error> {
        let (aps_metadata, zcl_metadata, command) = datagram.into_parts();
        let source_endpoint = self
            .source_endpoint(aps_metadata, zcl_metadata.direction)
            .await?;
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

        let transmission = match self
            .aps
            .transmit(destination, source_endpoint, aps_metadata, payload)
            .await
        {
            Ok(transmission) => transmission,
            Err(error) => {
                self.responses.remove(&index);
                return Err(error.into());
            }
        };

        Ok(ApsProtocolResponse::new(transmission, rx))
    }

    /// Return a local endpoint that advertises the profile, cluster, and ZCL role.
    async fn source_endpoint(
        &mut self,
        aps_metadata: crate::aps::Metadata,
        direction: Direction,
    ) -> Result<Endpoint, Error> {
        if self.endpoints.is_none() {
            self.endpoints = Some(self.ncp.get_endpoints().await?);
        }

        self.endpoints
            .as_deref()
            .and_then(|endpoints| {
                Self::matching_source_endpoint(endpoints, aps_metadata, direction)
            })
            .ok_or_else(|| Self::no_source_endpoint_error(aps_metadata, direction))
    }

    /// Find a local application endpoint that advertises the outgoing command's ZCL role.
    fn matching_source_endpoint(
        endpoints: &[SimpleDescriptor],
        aps_metadata: crate::aps::Metadata,
        direction: Direction,
    ) -> Option<Endpoint> {
        endpoints.iter().find_map(|descriptor| {
            let endpoint = descriptor.endpoint();
            let clusters = match direction {
                Direction::ClientToServer => descriptor.output_clusters(),
                Direction::ServerToClient => descriptor.input_clusters(),
            };

            (matches!(endpoint, Endpoint::Application(_))
                && descriptor.profile_id() == aps_metadata.profile().as_u16()
                && clusters.contains(&aps_metadata.cluster_id()))
            .then_some(endpoint)
        })
    }

    /// Construct the error returned when no compatible local source endpoint exists.
    const fn no_source_endpoint_error(
        metadata: crate::aps::Metadata,
        direction: Direction,
    ) -> Error {
        Error::NoSourceEndpoint {
            profile: metadata.profile(),
            cluster_id: metadata.cluster_id(),
            direction,
        }
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
    pub fn spawn(ncp: NcpHandle, aps: Aps, events: Sender<Event>) -> Sender<Message> {
        let (zcl_tx, zcl_rx) = tokio::sync::mpsc::channel(MPSC_CHANNEL_SIZE);
        spawn(Self::new(ncp, aps, events).run(zcl_rx));
        zcl_tx
    }
}

#[cfg(test)]
mod tests {
    use std::future::{Future, poll_fn};
    use std::num::NonZeroUsize;
    use std::time::Duration;

    use bytes::Bytes;
    use tokio::runtime::Builder;
    use tokio::sync::mpsc::channel;
    use zb_aps::Data;
    use zb_aps::data::Header as ApsHeader;
    use zb_core::endpoint::Application;
    use zb_core::short_id::Device;
    use zb_core::{Cluster as ClusterId, Destination, Direction, Endpoint, IeeeAddress, Profile};
    use zb_hw::{
        ChannelMask, Driver, Error as HardwareError, FoundNetwork, NcpHandle, Operation,
        ScanDuration, ScannedChannel,
    };
    use zb_nwk::Source;
    use zb_zcl::on_off::{Command as OnOffCommand, On};
    use zb_zcl::{Cluster, Command, Frame, Header as ZclHeader, Scope};
    use zb_zdp::{AppFlags, SimpleDescriptor};

    use super::{Message, Subscription, SubscriptionFilter, Transceiver};
    use crate::MPSC_CHANNEL_SIZE;
    use crate::aps::Aps;

    const SOURCE_NODE_ID: u16 = 0x4321;
    const TRANSACTION_SEQUENCE: u8 = 7;
    const APS_COUNTER: u8 = 9;
    const LOCAL_ENDPOINT_ID: u8 = 0x0B;
    const DEVICE_ID: u16 = 0x0100;
    const NCP_CHANNEL_SIZE: NonZeroUsize = NonZeroUsize::MIN;

    #[derive(Debug)]
    struct TestDriver;

    impl Driver for TestDriver {
        async fn get_endpoints(&self) -> Result<Box<[SimpleDescriptor]>, HardwareError> {
            Ok(Box::default())
        }

        async fn get_pan_id(&mut self) -> Result<u16, HardwareError> {
            Err(HardwareError::Unsupported(Operation::GetPanId))
        }

        async fn get_ieee_address(&mut self) -> Result<IeeeAddress, HardwareError> {
            Err(HardwareError::Unsupported(Operation::GetIeeeAddress))
        }

        async fn scan_networks(
            &mut self,
            _channel_mask: ChannelMask,
            _duration: ScanDuration,
        ) -> Result<Vec<FoundNetwork>, HardwareError> {
            Err(HardwareError::Unsupported(Operation::ScanNetworks))
        }

        async fn scan_channels(
            &mut self,
            _channel_mask: ChannelMask,
            _duration: ScanDuration,
        ) -> Result<Vec<ScannedChannel>, HardwareError> {
            Err(HardwareError::Unsupported(Operation::ScanChannels))
        }

        async fn allow_joins(&mut self, _duration: Duration) -> Result<Duration, HardwareError> {
            Err(HardwareError::Unsupported(Operation::AllowJoins))
        }

        async fn route_request(&mut self, _radius: u8) -> Result<(), HardwareError> {
            Err(HardwareError::Unsupported(Operation::RouteRequest))
        }

        async fn short_id_to_ieee_address(
            &mut self,
            _short_id: Device,
        ) -> Result<IeeeAddress, HardwareError> {
            Err(HardwareError::Unsupported(Operation::ShortIdToIeeeAddress))
        }

        async fn ieee_address_to_short_id(
            &mut self,
            _ieee_address: IeeeAddress,
        ) -> Result<Device, HardwareError> {
            Err(HardwareError::Unsupported(Operation::IeeeAddressToShortId))
        }

        async fn transmit(
            &mut self,
            _destination: Destination,
            _frame: Data<Bytes>,
        ) -> Result<(), HardwareError> {
            Err(HardwareError::Unsupported(Operation::Transmit))
        }
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
                let (ncp, ncp_actor) = test_ncp();
                tokio::spawn(ncp_actor);
                tokio::spawn(Transceiver::new(ncp, Aps::new(aps_sender), events).run(messages));
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

    fn test_ncp() -> (NcpHandle, impl Future<Output = TestDriver> + Send) {
        TestDriver.into_actor(NCP_CHANNEL_SIZE)
    }

    #[test]
    fn selects_advertised_endpoint_for_outgoing_zcl_role() {
        let descriptor = local_descriptor();
        let endpoints = [descriptor];

        let client_endpoint = Transceiver::matching_source_endpoint(
            &endpoints,
            crate::aps::Metadata::new(Profile::ZigbeeHomeAutomation, ClusterId::OnOff.as_u16()),
            Direction::ClientToServer,
        );
        let server_endpoint = Transceiver::matching_source_endpoint(
            &endpoints,
            crate::aps::Metadata::new(
                Profile::ZigbeeHomeAutomation,
                ClusterId::OtaUpgrade.as_u16(),
            ),
            Direction::ServerToClient,
        );

        assert_eq!(client_endpoint, Some(Endpoint::from(LOCAL_ENDPOINT_ID)));
        assert_eq!(server_endpoint, Some(Endpoint::from(LOCAL_ENDPOINT_ID)));
    }

    #[test]
    fn rejects_endpoint_without_matching_zcl_role() {
        let endpoints = [local_descriptor()];
        let endpoint = Transceiver::matching_source_endpoint(
            &endpoints,
            crate::aps::Metadata::new(Profile::ZigbeeHomeAutomation, ClusterId::OnOff.as_u16()),
            Direction::ServerToClient,
        );

        assert_eq!(endpoint, None);
    }

    fn local_descriptor() -> SimpleDescriptor {
        SimpleDescriptor::new(
            Endpoint::from(LOCAL_ENDPOINT_ID),
            Profile::ZigbeeHomeAutomation,
            DEVICE_ID,
            AppFlags::empty(),
            std::iter::once(ClusterId::OtaUpgrade.as_u16()).collect(),
            std::iter::once(ClusterId::OnOff.as_u16()).collect(),
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
}
