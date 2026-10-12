use std::num::NonZeroUsize;
use std::sync::Mutex;
use std::time::Duration;

use bytes::Bytes;
use le_stream::ToLeStream;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{WeakSender, channel};
use tokio::sync::oneshot;
use tokio::sync::oneshot::{Receiver, Sender};
use tokio::time::timeout;
use zb_aps::apsde::{
    ConfirmStatus, DataIndication, DataRequest, IndicationMetadata, IndicationStatus,
    IndividualEndpoint, NetworkAddress, NetworkDestination, ReceivedDestination,
    RequestDestination, Security, Source, Status as ApsStatus,
};
use zb_core::node::Descriptor;
use zb_core::short_id::Device;
use zb_core::{ClusterSpecific, Endpoint, IeeeAddress, Profile};
use zb_hw::{
    ChannelMask, Driver, Error as HardwareError, FoundNetwork, Operation, ScanDuration,
    ScannedChannel, TransmissionError,
};
use zb_zdp::{ActiveEpReq, Command, DeviceAndServiceDiscovery, Frame, SimpleDescriptor, Status};

use super::server::{permit_joining_response, track_reply_completion};
use super::{Message, Transceiver};
use crate::Error;
use crate::aps::{Aps, Message as ApsMessage, Metadata, TransmissionResponse};
use crate::event::EventSink;
use crate::zdp::ZdpKey;

const NO_WORK_LIMIT: usize = 0;
const SEQUENCE_OFFSET: usize = 0;
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
    started: Mutex<Option<Sender<()>>>,
    release: Mutex<Option<Receiver<()>>>,
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
        _request: DataRequest<Bytes>,
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
            let message = tracked_reply_failure(TransmissionError::Rejected.into()).await;

            assert!(matches!(
                message,
                Message::ReplyTransmissionFailed {
                    error: zb_hw::Error::Transmission(TransmissionError::Rejected)
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
                tracked_reply_failure(TransmissionError::Confirmation(status).into()).await;

            assert!(matches!(
                message,
                Message::ReplyTransmissionFailed {
                    error: zb_hw::Error::Transmission(
                        TransmissionError::Confirmation(received)
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
                &crate::CoordinatorConfig::default(),
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
            let ApsMessage::Transmit { request, response } = aps_messages
                .recv()
                .await
                .expect("submission reaches the APS actor")
            else {
                panic!("expected APS transmission");
            };
            let original_sequence = request.asdu()[SEQUENCE_OFFSET];
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
                    TransmissionError::NoRoute
                ))))
            ));
            assert!(transceiver.handle_actor_message(stale_completion));

            let request = communication_request(device);
            let (next_sequence, token, _response) = transceiver
                .responses
                .register(|sequence| ZdpKey::from_command(device, sequence, &request))
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

#[test]
fn server_operation_admission_recovers_and_ignores_stale_completions() {
    Runtime::new().unwrap().block_on(async {
        let (aps, aps_messages) = channel(CHANNEL_SIZE);
        drop(aps_messages);
        let (inbox, mut messages) = channel(CHANNEL_SIZE);
        let mut actor = bookkeeping_actor(Aps::new(aps), inbox.downgrade());
        actor.server_operations.set_limit(NO_WORK_LIMIT);
        actor.handle_message_received(active_endpoint_request(), true);
        assert!(actor.server_operations.is_empty());

        actor.server_operations.set_limit(CHANNEL_SIZE);
        actor.handle_message_received(active_endpoint_request(), true);
        actor.handle_message_received(active_endpoint_request(), true);
        let Message::ServerOperationFinished { id: previous } =
            timeout(TEST_TIMEOUT, messages.recv())
                .await
                .unwrap()
                .unwrap()
        else {
            panic!("expected server completion")
        };
        actor.handle_actor_message(Message::ServerOperationFinished { id: previous });
        assert!(
            actor.server_operations.is_empty(),
            "only one request was admitted"
        );

        actor.handle_message_received(active_endpoint_request(), true);
        let Message::ServerOperationFinished { id: current } =
            timeout(TEST_TIMEOUT, messages.recv())
                .await
                .unwrap()
                .unwrap()
        else {
            panic!("expected server completion")
        };
        assert_ne!(previous, current);
        actor.handle_actor_message(Message::ServerOperationFinished { id: previous });
        assert!(!actor.server_operations.is_empty());
        actor.handle_actor_message(Message::ServerOperationFinished { id: current });
        assert!(actor.server_operations.is_empty());
    });
}

#[test]
fn submission_admission_recovers_after_failure_and_shutdown_fails_pending_callers() {
    Runtime::new().unwrap().block_on(async {
        let (aps, mut aps_messages) = channel(CHANNEL_SIZE);
        let (inbox, mut messages) = channel(CHANNEL_SIZE);
        let mut actor = bookkeeping_actor(Aps::new(aps), inbox.downgrade());
        let device = Device::new(REMOTE_ADDRESS).unwrap();
        actor.submissions.set_limit(NO_WORK_LIMIT);
        let (response, result) = oneshot::channel();
        actor.communicate(device, communication_request(device), response);
        assert!(matches!(result.await, Ok(Err(Error::SendError))));
        assert!(aps_messages.try_recv().is_err());

        actor.submissions.set_limit(CHANNEL_SIZE);
        let (response, first_result) = oneshot::channel();
        actor.communicate(device, communication_request(device), response);
        let (response, rejected_result) = oneshot::channel();
        actor.communicate(device, communication_request(device), response);
        assert!(matches!(rejected_result.await, Ok(Err(Error::SendError))));
        let ApsMessage::Transmit { response, .. } = timeout(TEST_TIMEOUT, aps_messages.recv())
            .await
            .unwrap()
            .unwrap()
        else {
            panic!("expected APS handoff")
        };
        response.send(Err(Error::SendError)).unwrap();
        let completion = timeout(TEST_TIMEOUT, messages.recv())
            .await
            .unwrap()
            .unwrap();
        actor.handle_actor_message(completion);
        assert!(matches!(first_result.await, Ok(Err(Error::SendError))));
        assert!(actor.submissions.has_capacity());

        let (response, pending_result) = oneshot::channel();
        actor.communicate(device, communication_request(device), response);
        assert!(!actor.submissions.has_capacity());
        assert!(!actor.handle_actor_message(Message::HardwareUnavailable));
        assert!(matches!(
            pending_result.await,
            Ok(Err(Error::Hardware(HardwareError::ActorUnavailable)))
        ));
        assert!(actor.submissions.has_capacity());
    });
}

/// An actor whose unused hardware receiver is closed; APS is controlled by each test.
fn bookkeeping_actor(aps: Aps, inbox: WeakSender<Message>) -> Transceiver {
    let (started, _started) = oneshot::channel();
    let (_release, release) = oneshot::channel();
    let (ncp, _driver) = DelayedEndpointDriver {
        started: Mutex::new(Some(started)),
        release: Mutex::new(Some(release)),
    }
    .into_actor(NCP_CHANNEL_SIZE);
    let (events, _events) = channel(CHANNEL_SIZE);
    Transceiver::new(
        ncp,
        aps,
        EventSink::new(events),
        Descriptor::default(),
        inbox,
    )
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
    let metadata = IndicationMetadata {
        destination: ReceivedDestination::Network {
            address: network_address(LOCAL_ADDRESS),
            endpoint: data_endpoint(),
        },
        source: Source::Network {
            address: network_address(REMOTE_ADDRESS),
            endpoint: data_endpoint(),
        },
        profile_id: Profile::Network.as_u16(),
        cluster_id: command.cluster_id(),
        status: IndicationStatus::success(),
        security: Security::<()>::Unsecured,
        link_quality: LINK_QUALITY,
        rx_time: (),
    };

    DataIndication::new(metadata, Frame::new(SEQUENCE, command))
}

fn communication_request(device: Device) -> DataRequest<Bytes> {
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
