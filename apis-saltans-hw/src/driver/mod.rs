#![cfg(feature = "driver")]

//! An interface for communicating with a Zigbee NCP (Network Co-Processor) device.

use std::time::Duration;

use apis_saltans_core::{Destination, IeeeAddress};
use tokio::task::JoinHandle;

pub use self::bridge::bridge;
pub use self::builder::Builder;
pub use self::event_translator::EventTranslator;
pub use self::initialize::Initialize;
pub use self::prepared_hardware::PreparedHardware;
use self::sealed_driver::SealedDriver;
use crate::common::{Datagram, Error, FoundNetwork, NcpHandle, ScannedChannel};

mod bridge;
mod builder;
mod event_translator;
mod initialize;
mod prepared_hardware;
mod sealed_driver;

/// A common Zigbee NCP driver interface.
pub trait Driver {
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
    /// - `channel_mask`: A bitmask representing the channels to scan.
    /// - `duration`: The duration to scan each channel. The meaning is implementation-specific.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn scan_networks(
        &mut self,
        channel_mask: u32,
        duration: u8,
    ) -> impl Future<Output = Result<Vec<FoundNetwork>, Error>> + Send;

    /// Scan channels for activity.
    ///
    /// # Parameters
    ///
    /// - `channel_mask`: A bitmask representing the channels to scan.
    /// - `duration`: The duration to scan each channel. The meaning is implementation-specific.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn scan_channels(
        &mut self,
        channel_mask: u32,
        duration: u8,
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
        short_id: u16,
    ) -> impl Future<Output = Result<IeeeAddress, Error>> + Send;

    /// Get the short ID of the device with the specified IEEE address.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn ieee_address_to_short_id(
        &mut self,
        ieee_address: IeeeAddress,
    ) -> impl Future<Output = Result<u16, Error>> + Send;

    /// Transmit an application datagram to the specified destination.
    ///
    /// # Errors
    ///
    /// Returns an error if the datagram cannot be transmitted.
    fn transmit(
        &mut self,
        destination: Destination,
        datagram: Datagram,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Spawn the actor in a tokio task.
    ///
    /// # Returns
    ///
    /// Returns a tuple of the tokio task's join handle and an actor proxy.
    fn spawn(self, channel_size: usize) -> (JoinHandle<Self>, NcpHandle)
    where
        Self: Sized + SealedDriver + 'static,
    {
        SealedDriver::spawn(self, channel_size)
    }
}
