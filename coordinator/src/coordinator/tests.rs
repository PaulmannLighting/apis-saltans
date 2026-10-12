use std::num::NonZeroUsize;
use std::time::Duration;

use bytes::Bytes;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{Sender, channel};
use tokio::time::timeout;
use zb_aps::apsde::{
    ConfirmStatus, DataConfirm, DataRequest, Destination, IndividualEndpoint, NetworkAddress,
    NetworkDestination, RequestDestination,
};
use zb_core::endpoint::Application;
use zb_core::node::Descriptor;
use zb_core::short_id::Device;
use zb_core::{Endpoint, IeeeAddress};
use zb_hw::{
    ApsdeEvent, ChannelMask, Driver, Error as HardwareError, Event as HardwareEvent, FoundNetwork,
    Operation, ScanDuration, ScannedChannel,
};
use zb_zcl::groups::GetGroupMembership;
use zb_zdp::{ActiveEpReq, SimpleDescriptor};

use crate::{Coordinator, CoordinatorConfig, Error, RuntimeExit, Zcl, Zdp};

const ADDRESS: u16 = 0x1234;
const DEADLINE: Duration = Duration::from_secs(2);
const LONG_TIMEOUT: Duration = Duration::from_secs(60);
const CAPACITY: usize = 4;

struct TestDriver {
    events: Sender<HardwareEvent<(), ()>>,
}

impl Driver for TestDriver {
    async fn get_endpoints(&self) -> Result<Box<[SimpleDescriptor]>, HardwareError> {
        unsupported(Operation::GetEndpoints)
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
        request: DataRequest<Bytes>,
        counter: u8,
    ) -> Result<(), HardwareError> {
        let RequestDestination::Network { address, endpoint } = request.destination() else {
            panic!("test expects individual network requests");
        };
        self.events
            .send(HardwareEvent::Apsde(ApsdeEvent::DataConfirm {
                counter,
                confirmation: DataConfirm::new(
                    Destination::Network { address, endpoint },
                    request.source_endpoint(),
                    ConfirmStatus::success(),
                    (),
                ),
            }))
            .await
            .expect("coordinator receives confirmations");
        Ok(())
    }
}

fn unsupported<T>(operation: Operation) -> Result<T, HardwareError> {
    Err(HardwareError::Unsupported(operation))
}

fn destination() -> NetworkDestination {
    NetworkDestination::new(NetworkAddress::new(ADDRESS).unwrap(), source_endpoint())
}

fn source_endpoint() -> IndividualEndpoint {
    IndividualEndpoint::new(Endpoint::Application(Application::MIN)).unwrap()
}

#[test]
fn managed_start_applies_inbox_capacity_and_both_protocol_deadlines() {
    Runtime::new().unwrap().block_on(async {
        let (hardware, incoming) = channel(CAPACITY);
        let (events, _received) = channel(CAPACITY);
        let (ncp, driver) = TestDriver { events: hardware }.into_actor(NonZeroUsize::MIN);
        let driver = tokio::spawn(driver);
        let config = CoordinatorConfig {
            inbox_capacity: NonZeroUsize::MIN,
            protocol_response_timeout: Duration::ZERO,
            ..CoordinatorConfig::default()
        };
        let (coordinator, runtime) =
            Coordinator::start_managed(ncp, Descriptor::default(), incoming, events, config)
                .unwrap();
        assert_eq!(coordinator.zcl.max_capacity(), NonZeroUsize::MIN.get());
        assert_eq!(coordinator.zdp.max_capacity(), NonZeroUsize::MIN.get());
        assert_eq!(coordinator.ota.max_capacity(), NonZeroUsize::MIN.get());
        let zcl = coordinator
            .communicate_command(
                destination().into(),
                source_endpoint(),
                GetGroupMembership::default(),
            )
            .await
            .unwrap();
        assert!(matches!(
            timeout(DEADLINE, zcl).await.unwrap(),
            Err(Error::ProtocolResponseTimeout)
        ));
        let zdp = Zdp::communicate(
            &coordinator,
            Device::new(ADDRESS).unwrap(),
            ActiveEpReq::new(ADDRESS),
        )
        .await
        .unwrap();
        assert!(matches!(
            timeout(DEADLINE, zdp).await.unwrap(),
            Err(Error::ProtocolResponseTimeout)
        ));
        assert_eq!(
            timeout(DEADLINE, runtime.shutdown()).await.unwrap(),
            RuntimeExit::Shutdown
        );
        drop(coordinator);
        timeout(DEADLINE, driver).await.unwrap().unwrap();
    });
}

#[test]
fn managed_shutdown_fails_a_pending_exchange_without_waiting_for_its_deadline() {
    Runtime::new().unwrap().block_on(async {
        let (hardware, incoming) = channel(CAPACITY);
        let (events, _received) = channel(CAPACITY);
        let (ncp, driver) = TestDriver { events: hardware }.into_actor(NonZeroUsize::MIN);
        let driver = tokio::spawn(driver);
        let config = CoordinatorConfig {
            protocol_response_timeout: LONG_TIMEOUT,
            ..CoordinatorConfig::default()
        };
        let (coordinator, runtime) =
            Coordinator::start_managed(ncp, Descriptor::default(), incoming, events, config)
                .unwrap();
        let response = coordinator
            .communicate_command(
                destination().into(),
                source_endpoint(),
                GetGroupMembership::default(),
            )
            .await
            .unwrap();
        assert_eq!(
            timeout(DEADLINE, runtime.shutdown()).await.unwrap(),
            RuntimeExit::Shutdown
        );
        assert!(timeout(DEADLINE, response).await.unwrap().is_err());
        drop(coordinator);
        timeout(DEADLINE, driver).await.unwrap().unwrap();
    });
}
