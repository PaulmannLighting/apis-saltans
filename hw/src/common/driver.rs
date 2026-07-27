use std::num::NonZeroUsize;
use std::time::Duration;

use bytes::Bytes;
use tokio::sync::mpsc::Receiver;
use zb_aps::Data;
use zb_core::short_id::Device;
use zb_core::{Destination, IeeeAddress};
use zb_zdp::SimpleDescriptor;

use crate::common::message::Message;
use crate::{ChannelMask, Error, FoundNetwork, NcpHandle, ScanDuration, ScannedChannel};

/// A common Zigbee NCP driver interface.
pub trait Driver: Send + 'static {
    /// Return the local application endpoints provided by the NCP.
    ///
    /// Every driver must implement this method and return a [`SimpleDescriptor`] for each endpoint
    /// exposed to the Zigbee network. Each descriptor supplies the endpoint ID, profile, device ID,
    /// application version, and input and output cluster lists used by coordinator-level ZDP and
    /// binding operations.
    ///
    /// # Errors
    ///
    /// Returns an error if the NCP cannot provide its local endpoint descriptors.
    fn get_endpoints(&self) -> impl Future<Output = Result<Box<[SimpleDescriptor]>, Error>> + Send;

    /// Get the PAN ID of the network.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn get_pan_id(&mut self) -> impl Future<Output = Result<u16, Error>> + Send;

    /// Get the IEEE address of the coordinator.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn get_ieee_address(&mut self) -> impl Future<Output = Result<IeeeAddress, Error>> + Send;

    /// Scan for available networks.
    ///
    /// # Parameters
    ///
    /// - `channel_mask`: Validated channel-page-zero channels to scan.
    /// - `duration`: Zigbee scan-duration exponent.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn scan_networks(
        &mut self,
        channel_mask: ChannelMask,
        duration: ScanDuration,
    ) -> impl Future<Output = Result<Vec<FoundNetwork>, Error>> + Send;

    /// Scan channels for activity.
    ///
    /// # Parameters
    ///
    /// - `channel_mask`: Validated channel-page-zero channels to scan.
    /// - `duration`: Zigbee scan-duration exponent.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn scan_channels(
        &mut self,
        channel_mask: ChannelMask,
        duration: ScanDuration,
    ) -> impl Future<Output = Result<Vec<ScannedChannel>, Error>> + Send;

    /// Allow devices to join the network for the specified duration.
    ///
    /// # Returns
    ///
    /// Returns the actual duration for which joining is allowed.
    /// This may be less than the requested duration if the requested
    /// duration is longer than the maximum allowed duration.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn allow_joins(
        &mut self,
        duration: Duration,
    ) -> impl Future<Output = Result<Duration, Error>> + Send;

    /// Send a route request.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn route_request(&mut self, radius: u8) -> impl Future<Output = Result<(), Error>> + Send;

    /// Get the IEEE address of the device with the specified short ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn short_id_to_ieee_address(
        &mut self,
        short_id: Device,
    ) -> impl Future<Output = Result<IeeeAddress, Error>> + Send;

    /// Get the short ID of the device with the specified IEEE address.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn ieee_address_to_short_id(
        &mut self,
        ieee_address: IeeeAddress,
    ) -> impl Future<Output = Result<Device, Error>> + Send;

    /// Start transmitting an APS data frame to the specified destination.
    ///
    /// Returning success means the backend accepted the frame. If the frame
    /// requests an APS acknowledgement, the backend later reports completion
    /// through [`crate::ApsEvent::Ack`] or [`crate::ApsEvent::Nak`] using the
    /// frame's APS counter.
    ///
    /// # Errors
    ///
    /// Returns an error if the backend does not accept the frame.
    fn transmit(
        &mut self,
        destination: Destination,
        frame: Data<Bytes>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Convert this driver into an actor handle and its driving future.
    ///
    /// The returned future must be spawned or otherwise continuously polled.
    /// It resolves with the driver after every strong [`NcpHandle`] has been
    /// dropped.
    fn into_actor(
        self,
        channel_capacity: NonZeroUsize,
    ) -> (NcpHandle, impl Future<Output = Self> + Send)
    where
        Self: Sized,
    {
        let (handle, receiver) = NcpHandle::channel(channel_capacity);
        (handle, serve(self, receiver))
    }
}

async fn serve<T>(mut driver: T, mut receiver: Receiver<Message>) -> T
where
    T: Driver,
{
    while let Some(message) = receiver.recv().await {
        match message {
            Message::GetEndpoints { response } => {
                response
                    .send(driver.get_endpoints().await)
                    .unwrap_or_else(drop);
            }
            Message::GetPanId { response } => {
                response
                    .send(driver.get_pan_id().await)
                    .unwrap_or_else(drop);
            }
            Message::GetIeeeAddress { response } => {
                response
                    .send(driver.get_ieee_address().await)
                    .unwrap_or_else(drop);
            }
            Message::ScanNetworks {
                channel_mask,
                duration,
                response,
            } => {
                response
                    .send(driver.scan_networks(channel_mask, duration).await)
                    .unwrap_or_else(drop);
            }
            Message::ScanChannels {
                channel_mask,
                duration,
                response,
            } => {
                response
                    .send(driver.scan_channels(channel_mask, duration).await)
                    .unwrap_or_else(drop);
            }
            Message::AllowJoins { duration, response } => {
                response
                    .send(driver.allow_joins(duration).await)
                    .unwrap_or_else(drop);
            }
            Message::RouteRequest { radius, response } => {
                response
                    .send(driver.route_request(radius).await)
                    .unwrap_or_else(drop);
            }
            Message::TranslateIeeeAddress { short_id, response } => {
                response
                    .send(driver.short_id_to_ieee_address(short_id).await)
                    .unwrap_or_else(drop);
            }
            Message::TranslateShortId {
                ieee_address,
                response,
            } => {
                response
                    .send(driver.ieee_address_to_short_id(ieee_address).await)
                    .unwrap_or_else(drop);
            }
            Message::Transmit {
                destination,
                frame,
                response,
            } => {
                response
                    .send(driver.transmit(destination, frame).await)
                    .unwrap_or_else(drop);
            }
        }
    }

    driver
}

#[cfg(all(test, feature = "coordinator"))]
mod tests {
    use std::num::NonZeroUsize;
    use std::time::Duration;

    use bytes::Bytes;
    use tokio::runtime::Builder;
    use zb_aps::Data;
    use zb_aps::data::Header;
    use zb_core::destination::{Broadcast, Destination};
    use zb_core::short_id::{Broadcast as BroadcastAddress, Device};
    use zb_core::{Endpoint, IeeeAddress, Profile};
    use zb_zdp::SimpleDescriptor;

    use super::Driver;
    use crate::{ChannelMask, Error, FoundNetwork, Operation, ScanDuration, ScannedChannel};

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

        async fn short_id_to_ieee_address(
            &mut self,
            _short_id: Device,
        ) -> Result<IeeeAddress, Error> {
            Ok(IEEE_ADDRESS)
        }

        async fn ieee_address_to_short_id(
            &mut self,
            _ieee_address: IeeeAddress,
        ) -> Result<Device, Error> {
            Device::new(DEVICE_SHORT_ID).ok_or(Error::Unsupported(Operation::IeeeAddressToShortId))
        }

        async fn transmit(
            &mut self,
            _destination: Destination,
            frame: Data<Bytes>,
        ) -> Result<(), Error> {
            if self.reject_transmission {
                Err(Error::Unsupported(Operation::Transmit))
            } else {
                self.transmitted_counter = Some(frame.header().counter());
                Ok(())
            }
        }
    }

    fn destination() -> Destination {
        Broadcast::new(BroadcastAddress::AllDevices, Endpoint::Broadcast).into()
    }

    fn frame(destination: Destination) -> Data<Bytes> {
        Data::new(
            Header::new(
                destination.into(),
                CLUSTER_ID,
                PROFILE_ID.into(),
                SOURCE_ENDPOINT,
                APS_COUNTER,
                None,
            ),
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

                let destination = destination();
                handle
                    .transmit(destination, frame(destination))
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
                let destination = destination();

                assert!(matches!(
                    handle.transmit(destination, frame(destination)).await,
                    Err(Error::Unsupported(Operation::Transmit))
                ));

                drop(handle);
                let driver = task.await.expect("actor task must finish");
                assert!(driver.transmitted_counter.is_none());
            });
    }
}
