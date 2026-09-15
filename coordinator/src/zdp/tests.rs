use std::num::NonZeroUsize;
use std::sync::Mutex;
use std::time::Duration;

use bytes::Bytes;
use le_stream::ToLeStream;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::channel;
use tokio::sync::oneshot;
use tokio::time::timeout;
use zb_aps::apsde::{
    ConfirmStatus, DataIndication, IndicationMetadata, IndicationStatus, IndividualEndpoint,
    NetworkAddress, NetworkDestination, ReceivedDestination, RequestDestination, Security, Source,
    Status as ApsStatus,
};
use zb_core::node::Descriptor;
use zb_core::short_id::Device;
use zb_core::{ClusterSpecific, Endpoint, IeeeAddress, Profile};
use zb_hw::{
    ChannelMask, Driver, Error as HardwareError, FoundNetwork, Operation, ScanDuration,
    ScannedChannel,
};
use zb_zdp::{ActiveEpReq, Command, DeviceAndServiceDiscovery, Frame, SimpleDescriptor, Status};

use super::server::{permit_joining_response, track_reply_completion};
use super::{Message, Transceiver};
use crate::Error;
use crate::aps::{Aps, Message as ApsMessage, Metadata, TransmissionResponse};
use crate::correlation::Key;
use crate::event::EventSink;

const CHANNEL_SIZE: usize = 1;
const APS_COUNTER: u8 = 1;
const LINK_QUALITY: u8 = u8::MAX;
const LOCAL_ADDRESS: u16 = 0;
const REMOTE_ADDRESS: u16 = 1;
const SEQUENCE: u8 = 42;
const NCP_CHANNEL_SIZE: NonZeroUsize = NonZeroUsize::MIN;
const TEST_TIMEOUT: Duration = Duration::from_millis(100);

#[derive(Debug)]
struct DelayedEndpointDriver {
    started: Mutex<Option<oneshot::Sender<()>>>,
    release: Mutex<Option<oneshot::Receiver<()>>>,
}

impl Driver for DelayedEndpointDriver {
    async fn get_endpoints(&self) -> Result<Box<[SimpleDescriptor]>, HardwareError> {
        self.started
            .lock()
            .expect("test driver start lock is available")
            .take()
            .expect("test driver receives one endpoint query")
            .send(())
            .expect("test waits for the endpoint query");
        let release = self
            .release
            .lock()
            .expect("test driver release lock is available")
            .take()
            .expect("test driver receives one endpoint query");
        release
            .await
            .expect("test controls endpoint query completion");
        Ok(Vec::new().into_boxed_slice())
    }

    async fn get_pan_id(&mut self) -> Result<u16, HardwareError> {
        unsupported(Operation::GetPanId)
    }

    async fn get_ieee_address(&mut self) -> Result<IeeeAddress, HardwareError> {
        unsupported(Operation::GetIeeeAddress)
    }

    async fn scan_networks(
        &mut self,
        _channel_mask: ChannelMask,
        _duration: ScanDuration,
    ) -> Result<Vec<FoundNetwork>, HardwareError> {
        unsupported(Operation::ScanNetworks)
    }

    async fn scan_channels(
        &mut self,
        _channel_mask: ChannelMask,
        _duration: ScanDuration,
    ) -> Result<Vec<ScannedChannel>, HardwareError> {
        unsupported(Operation::ScanChannels)
    }

    async fn allow_joins(&mut self, _duration: Duration) -> Result<Duration, HardwareError> {
        unsupported(Operation::AllowJoins)
    }

    async fn route_request(&mut self, _radius: u8) -> Result<(), HardwareError> {
        unsupported(Operation::RouteRequest)
    }

    async fn short_id_to_ieee_address(
        &mut self,
        _short_id: Device,
    ) -> Result<IeeeAddress, HardwareError> {
        unsupported(Operation::ShortIdToIeeeAddress)
    }

    async fn ieee_address_to_short_id(
        &mut self,
        _ieee_address: IeeeAddress,
    ) -> Result<Device, HardwareError> {
        unsupported(Operation::IeeeAddressToShortId)
    }

    async fn transmit(
        &mut self,
        _request: zb_aps::apsde::DataRequest<Bytes>,
        _counter: u8,
    ) -> Result<(), HardwareError> {
        unsupported(Operation::Transmit)
    }
}

#[test]
fn permit_joining_rejects_unicast_and_ignores_broadcast_requests() {
    let response = permit_joining_response(false).expect("a unicast request requires a rejection");

    assert_eq!(response.status(), Ok(Status::InvalidRequestType));
    assert!(permit_joining_response(true).is_none());
}

#[test]
fn reports_deferred_backend_rejection_through_the_actor_inbox() {
    Runtime::new()
        .expect("runtime must be available")
        .block_on(async {
            let message = tracked_reply_failure(zb_hw::TransmissionError::Rejected.into()).await;

            assert!(matches!(
                message,
                Message::ReplyTransmissionFailed {
                    error: zb_hw::Error::Transmission(zb_hw::TransmissionError::Rejected)
                }
            ));
        });
}

#[test]
fn reports_deferred_acknowledgement_failure_through_the_actor_inbox() {
    Runtime::new()
        .expect("runtime must be available")
        .block_on(async {
            let status = ConfirmStatus::Aps(ApsStatus::NoAcknowledgement);
            let message =
                tracked_reply_failure(zb_hw::TransmissionError::Confirmation(status).into()).await;

            assert!(matches!(
                message,
                Message::ReplyTransmissionFailed {
                    error: zb_hw::Error::Transmission(
                        zb_hw::TransmissionError::Confirmation(received)
                    )
                } if received == status
            ));
        });
}

#[test]
fn actor_handles_hardware_shutdown_while_endpoint_query_is_pending() {
    Runtime::new()
        .expect("runtime must be available")
        .block_on(async {
            let (started, query_started) = oneshot::channel();
            let (release_query, release) = oneshot::channel();
            let (ncp, driver) = DelayedEndpointDriver {
                started: Mutex::new(Some(started)),
                release: Mutex::new(Some(release)),
            }
            .into_actor(NCP_CHANNEL_SIZE);
            let driver = tokio::spawn(driver);
            let (aps_messages, _aps_receiver) = channel(CHANNEL_SIZE);
            let (events, _event_receiver) = channel(CHANNEL_SIZE);
            let zdp = Transceiver::spawn(
                ncp,
                Aps::new(aps_messages),
                EventSink::new(events),
                Descriptor::default(),
            );

            zdp.send(Message::Received {
                indication: active_endpoint_request(),
                response_required: true,
            })
            .await
            .expect("ZDP actor accepts the request");
            timeout(TEST_TIMEOUT, query_started)
                .await
                .expect("endpoint query must start")
                .expect("endpoint query start sender remains available");
            zdp.send(Message::HardwareUnavailable)
                .await
                .expect("ZDP actor accepts hardware shutdown");
            timeout(TEST_TIMEOUT, zdp.closed())
                .await
                .expect("pending endpoint query must not block actor shutdown");

            release_query
                .send(())
                .expect("driver still waits for endpoint query completion");
            drop(zdp);
            timeout(TEST_TIMEOUT, driver)
                .await
                .expect("driver actor must stop after ZDP shutdown")
                .expect("driver actor task must not panic");
        });
}

#[test]
fn ignores_server_request_when_hardware_does_not_require_a_response() {
    let (started, _query_started) = oneshot::channel();
    let (_release_query, release) = oneshot::channel();
    let (ncp, _driver) = DelayedEndpointDriver {
        started: Mutex::new(Some(started)),
        release: Mutex::new(Some(release)),
    }
    .into_actor(NCP_CHANNEL_SIZE);
    let (aps_messages, _aps_receiver) = channel(CHANNEL_SIZE);
    let (events, _event_receiver) = channel(CHANNEL_SIZE);
    let (zdp_inbox, _zdp_messages) = channel(CHANNEL_SIZE);
    let mut transceiver = Transceiver::new(
        ncp,
        Aps::new(aps_messages),
        EventSink::new(events),
        Descriptor::default(),
        zdp_inbox.downgrade(),
    );

    transceiver.handle_message_received(active_endpoint_request(), false);

    assert!(transceiver.server_operations.is_empty());
}

#[test]
fn network_down_quarantines_a_submission_that_completed_outside_the_actor() {
    Runtime::new()
        .expect("runtime must be available")
        .block_on(async {
            let (started, _query_started) = oneshot::channel();
            let (_release_query, release) = oneshot::channel();
            let (ncp, driver) = DelayedEndpointDriver {
                started: Mutex::new(Some(started)),
                release: Mutex::new(Some(release)),
            }
            .into_actor(NCP_CHANNEL_SIZE);
            let driver = tokio::spawn(driver);
            let (aps_inbox, mut aps_messages) = channel(CHANNEL_SIZE);
            let (events, _event_receiver) = channel(CHANNEL_SIZE);
            let (zdp_inbox, mut zdp_messages) = channel(CHANNEL_SIZE);
            let mut transceiver = Transceiver::new(
                ncp,
                Aps::new(aps_inbox.clone()),
                EventSink::new(events),
                Descriptor::default(),
                zdp_inbox.downgrade(),
            );
            let device = Device::new(REMOTE_ADDRESS).expect("test device ID is valid");
            let request = communication_request(device);
            let (response, result) = oneshot::channel();

            transceiver.communicate(device, request, response);
            let original_sequence = transceiver
                .communication_submissions
                .values()
                .next()
                .expect("communication submission is tracked")
                .token
                .key()
                .sequence();
            let ApsMessage::Transmit { response, .. } = aps_messages
                .recv()
                .await
                .expect("submission reaches the APS actor")
            else {
                panic!("expected APS transmission");
            };
            let (_completion, deferred) = oneshot::channel();
            let transmission =
                TransmissionResponse::test_new(deferred, APS_COUNTER, aps_inbox.downgrade());
            response
                .send(Ok(transmission))
                .expect("submission task still waits for APS handoff");
            let stale_completion = timeout(TEST_TIMEOUT, zdp_messages.recv())
                .await
                .expect("submission completion must be queued")
                .expect("ZDP actor inbox remains available");

            assert!(transceiver.handle_actor_message(Message::NetworkDown));
            assert!(matches!(
                result.await,
                Ok(Err(Error::Hardware(HardwareError::Transmission(
                    zb_hw::TransmissionError::NoRoute
                ))))
            ));
            assert!(transceiver.handle_actor_message(stale_completion));

            let request = communication_request(device);
            let (next_sequence, token, _response) = transceiver
                .responses
                .register(|sequence| Key::from_zdp_command(device, sequence, &request))
                .expect("another transaction sequence remains available");
            assert_ne!(next_sequence, original_sequence);
            transceiver.responses.discard(token);

            drop(transceiver);
            drop(aps_inbox);
            drop(zdp_inbox);
            timeout(TEST_TIMEOUT, driver)
                .await
                .expect("driver actor must stop after transceiver shutdown")
                .expect("driver actor task must not panic");
        });
}

async fn tracked_reply_failure(error: zb_hw::Error) -> Message {
    let (aps_inbox, _aps_messages) = channel::<ApsMessage>(CHANNEL_SIZE);
    let (completion, result) = oneshot::channel();
    let transmission = TransmissionResponse::test_new(result, APS_COUNTER, aps_inbox.downgrade());
    let (zdp_inbox, mut zdp_messages) = channel(CHANNEL_SIZE);
    track_reply_completion(transmission, zdp_inbox.downgrade());

    completion
        .send(Err(error))
        .expect("transmission response must be waiting");
    zdp_messages
        .recv()
        .await
        .expect("deferred failure must reach the ZDP actor inbox")
}

fn active_endpoint_request() -> DataIndication<Frame<Command>, (), ()> {
    let command: Command = DeviceAndServiceDiscovery::from(ActiveEpReq::new(LOCAL_ADDRESS)).into();
    let metadata = IndicationMetadata::new(
        ReceivedDestination::Network {
            address: network_address(LOCAL_ADDRESS),
            endpoint: data_endpoint(),
        },
        Source::Network {
            address: network_address(REMOTE_ADDRESS),
            endpoint: data_endpoint(),
        },
        Profile::Network.as_u16(),
        command.cluster_id(),
        IndicationStatus::success(),
        Security::<()>::Unsecured,
        LINK_QUALITY,
        (),
    );

    DataIndication::new(metadata, Frame::new(SEQUENCE, command))
}

fn communication_request(device: Device) -> zb_aps::apsde::DataRequest<Bytes> {
    let destination: RequestDestination =
        NetworkDestination::new(network_address(device.as_u16()), data_endpoint()).into();
    crate::aps::data_request(
        destination,
        data_endpoint(),
        Metadata::new(Profile::Network, <ActiveEpReq as ClusterSpecific>::ID),
        ActiveEpReq::new(REMOTE_ADDRESS).to_le_stream().collect(),
    )
}

fn data_endpoint() -> IndividualEndpoint {
    IndividualEndpoint::new(Endpoint::Data).expect("data endpoint is individual")
}

fn network_address(address: u16) -> NetworkAddress {
    NetworkAddress::new(address).expect("test NWK address is valid")
}

fn unsupported<T>(operation: Operation) -> Result<T, HardwareError> {
    Err(HardwareError::Unsupported(operation))
}
