use std::num::NonZeroUsize;
use std::time::Duration;

use bytes::Bytes;
use tokio::sync::mpsc::Receiver;
use zb_aps::apsde::DataRequest;
use zb_core::IeeeAddress;
use zb_core::short_id::Device;
use zb_zdp::SimpleDescriptor;

pub use self::backup::{ActorBuilder, BackupDisabled, BackupEnabled, NetworkDump, NetworkRestore};
use self::backup::{DumpDispatch, RestoreDispatch};
use crate::common::message::Message;
use crate::{
    BackupCapabilities, ChannelMask, Error, FoundNetwork, NcpHandle, ScanDuration, ScannedChannel,
};

mod backup;

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

    /// Start transmitting an APS data-service request.
    ///
    /// Returning success means the backend accepted the request. If the request
    /// requests an APS acknowledgement, the backend later reports completion through
    /// [`crate::ApsdeEvent::DataConfirm`] using the APS counter supplied with the request.
    ///
    /// # Errors
    ///
    /// Returns an error if the backend does not accept the request.
    fn transmit(
        &mut self,
        request: DataRequest<Bytes>,
        counter: u8,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Configure optional network dump and restore dispatch for this driver actor.
    fn actor_builder(self, channel_capacity: NonZeroUsize) -> ActorBuilder<Self>
    where
        Self: Sized,
    {
        ActorBuilder::new(self, channel_capacity)
    }

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
        self.actor_builder(channel_capacity).build()
    }
}

async fn serve<T, Dump, Restore>(mut driver: T, mut receiver: Receiver<Message>) -> T
where
    T: Driver,
    Dump: DumpDispatch<T>,
    Restore: RestoreDispatch<T>,
{
    while let Some(message) = receiver.recv().await {
        match message {
            Message::GetBackupCapabilities { response } => {
                response
                    .send(Ok(BackupCapabilities::new(Dump::ENABLED, Restore::ENABLED)))
                    .unwrap_or_else(drop);
            }
            Message::DumpNetwork { response } => {
                response
                    .send(Dump::dump(&mut driver).await)
                    .unwrap_or_else(drop);
            }
            Message::RestoreNetwork { request, response } => {
                response
                    .send(Restore::restore(&mut driver, request.backup, request.options).await)
                    .unwrap_or_else(drop);
            }
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
                request,
                counter,
                response,
            } => {
                response
                    .send(driver.transmit(request, counter).await)
                    .unwrap_or_else(drop);
            }
        }
    }

    driver
}

#[cfg(all(test, feature = "coordinator"))]
mod tests;
