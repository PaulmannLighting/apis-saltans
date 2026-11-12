use std::num::NonZeroUsize;
use std::time::Duration;

use bytes::Bytes;
use tokio::runtime::Builder;
use zb_aps::apsde::{DataRequest, IndividualEndpoint, RequestDestination, RequestParameters};
use zb_core::short_id::{Broadcast, Device};
use zb_core::{Endpoint, IeeeAddress, Profile};
use zb_zdp::SimpleDescriptor;

use super::Driver;
use crate::{ChannelMask, Error, FoundNetwork, Operation, ScanDuration, ScannedChannel};

mod backup;

const ACTOR_CAPACITY: NonZeroUsize = NonZeroUsize::MIN;
const APS_COUNTER: u8 = 0;
const CLUSTER_ID: u16 = 0x0006;
const DEVICE_SHORT_ID: u16 = 0x1234;
const PAN_ID: u16 = 0xABCD;
const PROFILE_ID: Profile = Profile::ZigbeeHomeAutomation;
const SOURCE_ENDPOINT: Endpoint = Endpoint::Data;
const IEEE_ADDRESS: IeeeAddress = IeeeAddress::new(1, 2, 3, 4, 5, 6, 7, 8);

#[derive(Debug, Default)]
struct FakeDriver {
    reject_transmission: bool,
    transmitted_counter: Option<u8>,
}

impl Driver for FakeDriver {
    async fn get_endpoints(&self) -> Result<Box<[SimpleDescriptor]>, Error> {
        Ok(Vec::new().into_boxed_slice())
    }

    async fn get_pan_id(&mut self) -> Result<u16, Error> {
        Ok(PAN_ID)
    }

    async fn get_ieee_address(&mut self) -> Result<IeeeAddress, Error> {
        Ok(IEEE_ADDRESS)
    }

    async fn scan_networks(
        &mut self,
        _channel_mask: ChannelMask,
        _duration: ScanDuration,
    ) -> Result<Vec<FoundNetwork>, Error> {
        Ok(Vec::new())
    }

    async fn scan_channels(
        &mut self,
        _channel_mask: ChannelMask,
        _duration: ScanDuration,
    ) -> Result<Vec<ScannedChannel>, Error> {
        Ok(Vec::new())
    }

    async fn allow_joins(&mut self, duration: Duration) -> Result<Duration, Error> {
        Ok(duration)
    }

    async fn route_request(&mut self, _radius: u8) -> Result<(), Error> {
        Ok(())
    }

    async fn short_id_to_ieee_address(&mut self, _short_id: Device) -> Result<IeeeAddress, Error> {
        Ok(IEEE_ADDRESS)
    }

    async fn ieee_address_to_short_id(
        &mut self,
        _ieee_address: IeeeAddress,
    ) -> Result<Device, Error> {
        Device::new(DEVICE_SHORT_ID).ok_or(Error::Unsupported(Operation::IeeeAddressToShortId))
    }

    async fn transmit(&mut self, _request: DataRequest<Bytes>, counter: u8) -> Result<(), Error> {
        if self.reject_transmission {
            Err(Error::Unsupported(Operation::Transmit))
        } else {
            self.transmitted_counter = Some(counter);
            Ok(())
        }
    }
}

const fn request() -> DataRequest<Bytes> {
    DataRequest::new(
        RequestParameters {
            destination: RequestDestination::Broadcast {
                address: Broadcast::AllDevices,
                endpoint: Endpoint::Broadcast,
            },
            profile_id: PROFILE_ID.as_u16(),
            cluster_id: CLUSTER_ID,
            source_endpoint: IndividualEndpoint::new(SOURCE_ENDPOINT)
                .expect("the ZDO data endpoint is an individual endpoint"),
        },
        Bytes::new(),
    )
}

#[test]
fn actor_dispatches_queries_and_accepted_transmission() {
    Builder::new_current_thread()
        .build()
        .expect("runtime must be available")
        .block_on(async {
            let (handle, actor) = FakeDriver::default().into_actor(ACTOR_CAPACITY);
            let task = tokio::spawn(actor);

            assert_eq!(
                handle
                    .get_pan_id()
                    .await
                    .expect("fake driver must return PAN ID"),
                PAN_ID
            );

            handle
                .transmit(request(), APS_COUNTER)
                .await
                .expect("fake driver must accept transmission");

            let weak = handle.downgrade();
            assert!(weak.upgrade().is_some());
            drop(handle);

            let driver = task.await.expect("actor task must finish");
            assert_eq!(driver.transmitted_counter, Some(APS_COUNTER));
            assert!(weak.upgrade().is_none());
        });
}

#[test]
fn actor_reports_backend_rejection_before_completion() {
    Builder::new_current_thread()
        .build()
        .expect("runtime must be available")
        .block_on(async {
            let driver = FakeDriver {
                reject_transmission: true,
                transmitted_counter: None,
            };
            let (handle, actor) = driver.into_actor(ACTOR_CAPACITY);
            let task = tokio::spawn(actor);

            assert!(matches!(
                handle.transmit(request(), APS_COUNTER).await,
                Err(Error::Unsupported(Operation::Transmit))
            ));

            drop(handle);
            let driver = task.await.expect("actor task must finish");
            assert!(driver.transmitted_counter.is_none());
        });
}
