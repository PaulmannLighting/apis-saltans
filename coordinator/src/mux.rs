use bytes::Bytes;
use log::{trace, warn};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender};
use zb_aps::apsde::DataIndication;
use zb_aps::data::Frame;
use zb_core::destination;
use zb_hw::{
    ApsdeEvent as HardwareApsdeEvent, DeviceEvent as HardwareDeviceEvent, Event as HardwareEvent,
    NetworkEvent as HardwareNetworkEvent,
};

use self::aps_payload::ApsPayload;
use crate::{Device, Event as ApplicationEvent, Event, Network, NetworkError, aps, zcl, zdp};

mod aps_payload;

/// Event multiplexer.
#[derive(Debug)]
pub struct Mux {
    events: Sender<ApplicationEvent>,
    aps: aps::Aps,
    zcl: Sender<zcl::Message>,
    zdp: Sender<zdp::Message>,
}

impl Mux {
    /// Create a new multiplexer.
    pub const fn new(
        events: Sender<ApplicationEvent>,
        aps: aps::Aps,
        zcl: Sender<zcl::Message>,
        zdp: Sender<zdp::Message>,
    ) -> Self {
        Self {
            events,
            aps,
            zcl,
            zdp,
        }
    }

    /// Start the multiplexer.
    pub fn spawn<T, K>(
        hw_events: Receiver<HardwareEvent<T, K>>,
        events_out: Sender<ApplicationEvent>,
        aps: aps::Aps,
        zcl_tx: Sender<zcl::Message>,
        zdp_tx: Sender<zdp::Message>,
    ) where
        T: Send + 'static,
        K: Send + 'static,
    {
        spawn(Self::new(events_out, aps, zcl_tx, zdp_tx).run(hw_events));
    }

    /// Run the multiplexer.
    pub async fn run<T, K>(self, mut messages: Receiver<HardwareEvent<T, K>>)
    where
        T: Send,
        K: Send,
    {
        while let Some(event) = messages.recv().await {
            self.multiplex(event).await;
        }
    }

    async fn multiplex<T, K>(&self, event: HardwareEvent<T, K>) {
        match event {
            HardwareEvent::Network(event) => self.multiplex_network_event(event).await,
            HardwareEvent::Device(event) => self.multiplex_device_event(event).await,
            HardwareEvent::Apsde(event) => self.multiplex_apsde_event(event).await,
            _ => trace!("Ignoring unsupported hardware event"),
        }
    }

    async fn multiplex_network_event(&self, event: HardwareNetworkEvent) {
        match event {
            HardwareNetworkEvent::Up => {
                trace!("Network is up");
                self.events
                    .send(ApplicationEvent::Network(Network::Up))
                    .await
                    .unwrap_or_else(drop);
            }
            HardwareNetworkEvent::Down => {
                trace!("Network is down");
                self.events
                    .send(ApplicationEvent::Network(Network::Down))
                    .await
                    .unwrap_or_else(drop);
            }
            HardwareNetworkEvent::Opened => {
                trace!("Network has been opened");
                self.zdp
                    .send(zdp::Message::NetworkOpened)
                    .await
                    .unwrap_or_else(|error| {
                        trace!("Failed to send ZDP message: {error}");
                    });
                self.events
                    .send(ApplicationEvent::Network(Network::Opened))
                    .await
                    .unwrap_or_else(drop);
            }
            HardwareNetworkEvent::Closed => {
                trace!("Network has been closed");
                self.zdp
                    .send(zdp::Message::NetworkClosed)
                    .await
                    .unwrap_or_else(|error| {
                        trace!("Failed to send ZDP message: {error}");
                    });
                self.events
                    .send(ApplicationEvent::Network(Network::Closed))
                    .await
                    .unwrap_or_else(drop);
            }
            HardwareNetworkEvent::RouteError(error) => {
                trace!("Route error: {error}");
                self.events
                    .send(ApplicationEvent::Network(Network::Error(
                        NetworkError::Route(error),
                    )))
                    .await
                    .unwrap_or_else(drop);
            }
            _ => trace!("Ignoring unsupported hardware network event"),
        }
    }

    async fn multiplex_device_event(&self, event: HardwareDeviceEvent) {
        match event {
            HardwareDeviceEvent::Joined(address) => {
                trace!("Device joined: {address}");
                self.events
                    .send(ApplicationEvent::Device(Device::Joined(address)))
                    .await
                    .unwrap_or_else(drop);
            }
            HardwareDeviceEvent::Rejoined { address, secured } => {
                trace!("Device joined: {address} (secured: {secured})");
                self.events
                    .send(ApplicationEvent::Device(Device::Rejoined {
                        address,
                        secured,
                    }))
                    .await
                    .unwrap_or_else(drop);
            }
            HardwareDeviceEvent::Left(address) => {
                trace!("Device left: {address}");
                self.events
                    .send(ApplicationEvent::Device(Device::Left(address)))
                    .await
                    .unwrap_or_else(drop);
            }
            _ => trace!("Ignoring unsupported hardware device event"),
        }
    }

    async fn multiplex_apsde_event<T, K>(&self, event: HardwareApsdeEvent<T, K>) {
        match event {
            HardwareApsdeEvent::DataIndication(indication) => {
                self.handle_data_indication(indication).await;
            }
            HardwareApsdeEvent::DataConfirm {
                counter,
                confirmation,
            } => {
                let status = confirmation.status();
                trace!(
                    "APS data confirmation for counter {counter}, destination {:?}: {status}",
                    confirmation.destination()
                );
                self.aps
                    .confirm(counter, status)
                    .await
                    .unwrap_or_else(|error| {
                        trace!("Failed to forward APS data confirmation: {error}");
                    });
            }
            _ => trace!("Ignoring unsupported hardware APS event"),
        }
    }

    async fn handle_data_indication<T, K>(&self, indication: DataIndication<Bytes, T, K>) {
        let indication = indication.map_context(drop, drop);
        let metadata = indication.metadata();
        if !metadata.status().is_success() {
            warn!(
                "Discarding unsuccessful APS data indication: {:?}",
                metadata.status()
            );
            return;
        }

        let Some((source, header)) = crate::apsde::legacy_context(metadata) else {
            warn!("Discarding APS data indication with unsupported addressing");
            return;
        };
        let frame = Frame::new(header, indication.asdu().clone());

        match frame.parse() {
            Ok(frame) => {
                self.forward_received_message(source, indication, frame)
                    .await;
            }
            Err(error) => warn!("Failed to parse APS data indication: {error}"),
        }
    }

    async fn forward_received_message(
        &self,
        source: zb_nwk::Source,
        indication: DataIndication<Bytes, (), ()>,
        aps_frame: Frame<ApsPayload>,
    ) {
        let (header, payload) = aps_frame.into_parts();

        match payload {
            ApsPayload::Zcl(frame) => {
                let indication = indication.map_asdu(|_| frame);

                self.zcl
                    .send(zcl::Message::Received { indication })
                    .await
                    .unwrap_or_else(|error| {
                        trace!("Failed to send ZCL message: {error}");
                    });
            }
            ApsPayload::Zdp(frame) => {
                let indication = indication.map_asdu(|_| frame);

                self.zdp
                    .send(zdp::Message::Received { indication })
                    .await
                    .unwrap_or_else(|error| {
                        trace!("Failed to send ZDP message: {error}");
                    });
            }
            ApsPayload::KeepAlive => {
                let Ok(device_id) = source.node_id().try_into().inspect_err(|id| {
                    warn!("Keep-Alive packet from invalid device id: {id:#06X}");
                }) else {
                    return;
                };

                self.events
                    .send(Event::Device(Device::KeepAlive(destination::Device::new(
                        device_id,
                        header.source_endpoint(),
                    ))))
                    .await
                    .unwrap_or_else(drop);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use le_stream::ToLeStream;
    use tokio::runtime::Runtime;
    use tokio::sync::mpsc::channel;
    use zb_aps::apsde::{
        ConfirmStatus, DataConfirm, DataIndication, Destination, IndicationMetadata,
        IndicationStatus, IndividualEndpoint, NetworkAddress, ReceivedDestination, Security,
        Source as ApsdeSource,
    };
    use zb_core::endpoint::Application;
    use zb_core::{Cluster, ClusterSpecific, Direction, Endpoint, Profile};
    use zb_hw::ApsdeEvent;
    use zb_zcl::on_off::{Command as OnOffCommand, On};
    use zb_zcl::{Command, Frame as ZclFrame, Header as ZclHeader, Scope};
    use zb_zdp::{
        ActiveEpReq, Command as ZdpCommand, DeviceAndServiceDiscovery, Frame as ZdpFrame,
    };

    use super::Mux;
    use crate::aps::{Aps, Message as ApsMessage};
    use crate::{MPSC_CHANNEL_SIZE, zcl, zdp};

    const APS_COUNTER: u8 = 7;
    const LINK_QUALITY: u8 = 255;
    const LOCAL_ADDRESS: u16 = 0;
    const REMOTE_ADDRESS: u16 = 0x1234;
    const RX_TIME: u64 = 42;
    const TX_TIME: u64 = 43;
    const ZCL_SEQUENCE: u8 = 9;
    const ZDP_SEQUENCE: u8 = 10;

    #[test]
    fn routes_data_indication_to_zcl() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let (mux, _aps_messages, mut zcl_messages, _zdp_messages) = test_mux();
                let endpoint = application_endpoint();
                let source_endpoint = individual_endpoint();
                let metadata = IndicationMetadata::new(
                    ReceivedDestination::Network {
                        address: network_address(LOCAL_ADDRESS),
                        endpoint: source_endpoint,
                    },
                    ApsdeSource::Network {
                        address: network_address(REMOTE_ADDRESS),
                        endpoint: source_endpoint,
                    },
                    Profile::ZigbeeHomeAutomation.as_u16(),
                    Cluster::OnOff.as_u16(),
                    IndicationStatus::success(),
                    Security::<()>::Unsecured,
                    LINK_QUALITY,
                    RX_TIME,
                );
                let header = ZclHeader::new(
                    Scope::ClusterSpecific,
                    Direction::ClientToServer,
                    false,
                    None,
                    ZCL_SEQUENCE,
                    <On as Command>::ID,
                );
                let asdu = ZclFrame::new(header, On).to_le_stream().collect();

                mux.handle_data_indication(DataIndication::new(metadata, asdu))
                    .await;

                let zcl::Message::Received { indication } = zcl_messages
                    .recv()
                    .await
                    .expect("ZCL message must be routed")
                else {
                    panic!("expected received ZCL message");
                };
                assert!(matches!(
                    indication.metadata().source(),
                    ApsdeSource::Network { address, .. }
                        if address.as_u16() == REMOTE_ADDRESS
                ));
                assert!(matches!(
                    indication.metadata().destination(),
                    ReceivedDestination::Network { endpoint: destination_endpoint, .. }
                        if destination_endpoint.get() == endpoint
                ));
                assert_eq!(indication.metadata().link_quality(), LINK_QUALITY);
                assert_eq!(indication.metadata().rx_time(), &());
                assert!(matches!(
                    indication.asdu().payload(),
                    zb_zcl::Cluster::OnOff(OnOffCommand::On(_))
                ));
            });
    }

    #[test]
    fn routes_data_indication_to_zdp() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let (mux, _aps_messages, _zcl_messages, mut zdp_messages) = test_mux();
                let source_endpoint =
                    IndividualEndpoint::new(Endpoint::Data).expect("data endpoint is individual");
                let metadata = IndicationMetadata::new(
                    ReceivedDestination::Network {
                        address: network_address(LOCAL_ADDRESS),
                        endpoint: source_endpoint,
                    },
                    ApsdeSource::Network {
                        address: network_address(REMOTE_ADDRESS),
                        endpoint: source_endpoint,
                    },
                    Profile::Network.as_u16(),
                    <ActiveEpReq as ClusterSpecific>::ID,
                    IndicationStatus::success(),
                    Security::<()>::Unsecured,
                    LINK_QUALITY,
                    RX_TIME,
                );
                let request = ActiveEpReq::new(REMOTE_ADDRESS);
                let asdu = ZdpFrame::new(ZDP_SEQUENCE, request)
                    .to_le_stream()
                    .collect();

                mux.handle_data_indication(DataIndication::new(metadata, asdu))
                    .await;

                let zdp::Message::Received { indication } = zdp_messages
                    .recv()
                    .await
                    .expect("ZDP message must be routed")
                else {
                    panic!("expected received ZDP message");
                };
                assert_eq!(indication.metadata().link_quality(), LINK_QUALITY);
                assert_eq!(indication.metadata().rx_time(), &());
                assert!(matches!(
                    indication.asdu().data(),
                    ZdpCommand::DeviceAndServiceDiscovery(DeviceAndServiceDiscovery::ActiveEpReq(
                        _
                    ))
                ));
            });
    }

    #[test]
    fn routes_data_confirmation_to_aps() {
        Runtime::new()
            .expect("runtime must be available")
            .block_on(async {
                let (mux, mut aps_messages, _zcl_messages, _zdp_messages) = test_mux();
                let confirmation = DataConfirm::new(
                    Destination::Network {
                        address: network_address(REMOTE_ADDRESS),
                        endpoint: application_endpoint(),
                    },
                    individual_endpoint(),
                    ConfirmStatus::success(),
                    TX_TIME,
                );

                mux.multiplex_apsde_event(ApsdeEvent::<u64>::DataConfirm {
                    counter: APS_COUNTER,
                    confirmation,
                })
                .await;

                assert!(matches!(
                    aps_messages.recv().await,
                    Some(ApsMessage::Confirm {
                        counter: APS_COUNTER,
                        status
                    }) if status.is_success()
                ));
            });
    }

    fn test_mux() -> (
        Mux,
        tokio::sync::mpsc::Receiver<ApsMessage>,
        tokio::sync::mpsc::Receiver<zcl::Message>,
        tokio::sync::mpsc::Receiver<zdp::Message>,
    ) {
        let (events, _application_events) = channel(MPSC_CHANNEL_SIZE);
        let (aps_messages, aps_receiver) = channel(MPSC_CHANNEL_SIZE);
        let (zcl_messages, zcl_receiver) = channel(MPSC_CHANNEL_SIZE);
        let (zdp_messages, zdp_receiver) = channel(MPSC_CHANNEL_SIZE);
        (
            Mux::new(events, Aps::new(aps_messages), zcl_messages, zdp_messages),
            aps_receiver,
            zcl_receiver,
            zdp_receiver,
        )
    }

    const fn application_endpoint() -> Endpoint {
        Endpoint::Application(Application::MIN)
    }

    const fn individual_endpoint() -> IndividualEndpoint {
        IndividualEndpoint::new(application_endpoint()).expect("application endpoint is individual")
    }

    const fn network_address(address: u16) -> NetworkAddress {
        NetworkAddress::new(address).expect("test NWK address is valid")
    }
}
