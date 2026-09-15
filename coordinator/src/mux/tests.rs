use std::time::Duration;

use le_stream::ToLeStream;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::channel;
use tokio::time::timeout;
use zb_aps::apsde::{
    ConfirmStatus, DataConfirm, DataIndication, Destination, IndicationMetadata, IndicationStatus,
    IndividualEndpoint, NetworkAddress, ReceivedDestination, Security, Source as ApsdeSource,
};
use zb_core::endpoint::Application;
use zb_core::short_id::Broadcast;
use zb_core::{Cluster, ClusterSpecific, Direction, Endpoint, Profile};
use zb_hw::{ApsdeEvent, NetworkEvent};
use zb_zcl::on_off::{Command as OnOffCommand, On};
use zb_zcl::{Command, Frame as ZclFrame, Header as ZclHeader, Scope};
use zb_zdp::{ActiveEpReq, Command as ZdpCommand, DeviceAndServiceDiscovery, Frame as ZdpFrame};

use super::Mux;
use crate::aps::{Aps, Message as ApsMessage};
use crate::event::EventSink;
use crate::{Event, MPSC_CHANNEL_SIZE, Network, NetworkError, ota, zcl, zdp};

const TEST_TIMEOUT: Duration = Duration::from_millis(100);
const APPLICATION_EVENT_CHANNEL_SIZE: usize = 1;
const LINK_QUALITY: u8 = 255;
const LOCAL_ADDRESS: u16 = 0;
const REMOTE_ADDRESS: u16 = 0x1234;
const RX_TIME: u64 = 42;
const TX_TIME: u64 = 43;
const APS_COUNTER: u8 = 8;
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

            mux.handle_data_indication(DataIndication::new(metadata, asdu), false)
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
fn routes_broadcast_data_indication_to_zdp() {
    Runtime::new()
        .expect("runtime must be available")
        .block_on(async {
            let (mux, _aps_messages, _zcl_messages, mut zdp_messages) = test_mux();
            let source_endpoint =
                IndividualEndpoint::new(Endpoint::Data).expect("data endpoint is individual");
            let metadata = IndicationMetadata::new(
                ReceivedDestination::Broadcast {
                    address: Broadcast::RxOnWhenIdle,
                    endpoint: Endpoint::Data,
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

            mux.handle_data_indication(DataIndication::new(metadata, asdu), true)
                .await;

            let zdp::Message::Received {
                indication,
                response_required,
            } = zdp_messages
                .recv()
                .await
                .expect("ZDP message must be routed")
            else {
                panic!("expected received ZDP message");
            };
            assert!(response_required);
            assert!(matches!(
                indication.metadata().destination(),
                ReceivedDestination::Broadcast {
                    address: Broadcast::RxOnWhenIdle,
                    endpoint: Endpoint::Data
                }
            ));
            assert_eq!(indication.metadata().link_quality(), LINK_QUALITY);
            assert_eq!(indication.metadata().rx_time(), &());
            assert!(matches!(
                indication.asdu().data(),
                ZdpCommand::DeviceAndServiceDiscovery(DeviceAndServiceDiscovery::ActiveEpReq(_))
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

#[test]
fn full_application_event_channel_does_not_block_hardware_event_routing() {
    Runtime::new()
        .expect("runtime must be available")
        .block_on(async {
            let (events, _application_events) = channel(APPLICATION_EVENT_CHANNEL_SIZE);
            events
                .send(Event::Network(Network::Down))
                .await
                .expect("application event receiver remains available");
            let (aps_messages, mut aps_receiver) = channel(MPSC_CHANNEL_SIZE);
            let (ota_messages, _ota_receiver) = channel(MPSC_CHANNEL_SIZE);
            let (zcl_messages, _zcl_receiver) = channel(MPSC_CHANNEL_SIZE);
            let (zdp_messages, _zdp_receiver) = channel(MPSC_CHANNEL_SIZE);
            let mux = Mux::new(
                EventSink::new(events),
                Aps::new(aps_messages),
                ota_messages,
                zcl_messages,
                zdp_messages,
            );
            let confirmation = DataConfirm::new(
                Destination::Network {
                    address: network_address(REMOTE_ADDRESS),
                    endpoint: application_endpoint(),
                },
                individual_endpoint(),
                ConfirmStatus::success(),
                TX_TIME,
            );

            timeout(TEST_TIMEOUT, async {
                mux.multiplex_network_event(NetworkEvent::Up).await;
                mux.multiplex_apsde_event(ApsdeEvent::<u64>::DataConfirm {
                    counter: APS_COUNTER,
                    confirmation,
                })
                .await;
            })
            .await
            .expect("application backpressure must not block the mux");

            assert!(matches!(
                aps_receiver.recv().await,
                Some(ApsMessage::Confirm {
                    counter: APS_COUNTER,
                    status
                }) if status.is_success()
            ));
        });
}

#[test]
fn full_zdp_inbox_does_not_delay_aps_confirmation() {
    Runtime::new()
        .expect("runtime must be available")
        .block_on(async {
            let (events, _application_events) = channel(MPSC_CHANNEL_SIZE);
            let (aps_messages, mut aps_receiver) = channel(MPSC_CHANNEL_SIZE);
            let (ota_messages, _ota_receiver) = channel(MPSC_CHANNEL_SIZE);
            let (zcl_messages, _zcl_receiver) = channel(MPSC_CHANNEL_SIZE);
            let (zdp_messages, mut zdp_receiver) = channel(1);
            zdp_messages
                .send(zdp::Message::NetworkDown)
                .await
                .expect("ZDP inbox remains available");
            let mux = Mux::new(
                EventSink::new(events),
                Aps::new(aps_messages),
                ota_messages,
                zcl_messages,
                zdp_messages,
            );
            let source_endpoint =
                IndividualEndpoint::new(Endpoint::Data).expect("data endpoint is individual");
            let metadata = IndicationMetadata::new(
                ReceivedDestination::Broadcast {
                    address: Broadcast::RxOnWhenIdle,
                    endpoint: Endpoint::Data,
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
            let asdu = ZdpFrame::new(ZDP_SEQUENCE, ActiveEpReq::new(REMOTE_ADDRESS))
                .to_le_stream()
                .collect();
            let confirmation = DataConfirm::new(
                Destination::Network {
                    address: network_address(REMOTE_ADDRESS),
                    endpoint: application_endpoint(),
                },
                individual_endpoint(),
                ConfirmStatus::success(),
                TX_TIME,
            );

            timeout(TEST_TIMEOUT, async {
                mux.handle_data_indication(DataIndication::new(metadata, asdu), true)
                    .await;
                mux.multiplex_apsde_event(ApsdeEvent::<u64>::DataConfirm {
                    counter: APS_COUNTER,
                    confirmation,
                })
                .await;
            })
            .await
            .expect("ZDP backpressure must not block the hardware event mux");

            assert!(matches!(
                aps_receiver.recv().await,
                Some(ApsMessage::Confirm {
                    counter: APS_COUNTER,
                    status
                }) if status.is_success()
            ));
            assert!(matches!(
                zdp_receiver.recv().await,
                Some(zdp::Message::NetworkDown)
            ));
        });
}

#[test]
fn hardware_event_stream_closure_stops_every_actor_and_emits_a_failure() {
    Runtime::new()
        .expect("runtime must be available")
        .block_on(async {
            let (hardware_events, inbound) = channel::<zb_hw::Event<(), ()>>(MPSC_CHANNEL_SIZE);
            let (events, mut application_events) = channel(MPSC_CHANNEL_SIZE);
            let (aps_messages, mut aps_receiver) = channel(MPSC_CHANNEL_SIZE);
            let (ota_messages, mut ota_receiver) = channel(MPSC_CHANNEL_SIZE);
            let (zcl_messages, mut zcl_receiver) = channel(MPSC_CHANNEL_SIZE);
            let (zdp_messages, mut zdp_receiver) = channel(MPSC_CHANNEL_SIZE);
            let mux = Mux::new(
                EventSink::new(events),
                Aps::new(aps_messages),
                ota_messages,
                zcl_messages,
                zdp_messages,
            );
            drop(hardware_events);

            mux.run(inbound).await;

            assert!(matches!(
                aps_receiver.recv().await,
                Some(ApsMessage::HardwareUnavailable)
            ));
            assert!(matches!(
                ota_receiver.recv().await,
                Some(ota::Message::HardwareUnavailable)
            ));
            assert!(matches!(
                zcl_receiver.recv().await,
                Some(zcl::Message::HardwareUnavailable)
            ));
            assert!(matches!(
                zdp_receiver.recv().await,
                Some(zdp::Message::HardwareUnavailable)
            ));
            assert!(matches!(
                application_events.recv().await,
                Some(Event::Network(Network::Error(
                    NetworkError::HardwareEventStreamClosed
                )))
            ));
        });
}

#[test]
fn full_aps_inbox_does_not_delay_other_terminal_notifications() {
    Runtime::new()
        .expect("runtime must be available")
        .block_on(async {
            let (events, _application_events) = channel(MPSC_CHANNEL_SIZE);
            let (aps_messages, mut aps_receiver) = channel(1);
            let (ota_messages, mut ota_receiver) = channel(1);
            let (zcl_messages, mut zcl_receiver) = channel(1);
            let (zdp_messages, mut zdp_receiver) = channel(1);
            aps_messages
                .send(ApsMessage::NetworkDown)
                .await
                .expect("APS inbox remains available");
            let mux = Mux::new(
                EventSink::new(events),
                Aps::new(aps_messages),
                ota_messages,
                zcl_messages,
                zdp_messages,
            );

            let shutdown = tokio::spawn(async move {
                mux.hardware_event_stream_closed().await;
            });

            timeout(TEST_TIMEOUT, async {
                assert!(matches!(
                    ota_receiver.recv().await,
                    Some(ota::Message::HardwareUnavailable)
                ));
                assert!(matches!(
                    zcl_receiver.recv().await,
                    Some(zcl::Message::HardwareUnavailable)
                ));
                assert!(matches!(
                    zdp_receiver.recv().await,
                    Some(zdp::Message::HardwareUnavailable)
                ));
            })
            .await
            .expect("one full actor inbox must not delay the other terminal notifications");
            assert!(!shutdown.is_finished());

            assert!(matches!(
                aps_receiver.recv().await,
                Some(ApsMessage::NetworkDown)
            ));
            assert!(matches!(
                aps_receiver.recv().await,
                Some(ApsMessage::HardwareUnavailable)
            ));
            shutdown.await.expect("mux shutdown must complete");
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
    let (ota_messages, _ota_receiver) = channel(MPSC_CHANNEL_SIZE);
    let (zcl_messages, zcl_receiver) = channel(MPSC_CHANNEL_SIZE);
    let (zdp_messages, zdp_receiver) = channel(MPSC_CHANNEL_SIZE);
    (
        Mux::new(
            EventSink::new(events),
            Aps::new(aps_messages),
            ota_messages,
            zcl_messages,
            zdp_messages,
        ),
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
