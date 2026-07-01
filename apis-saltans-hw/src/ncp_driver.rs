//! An interface for communicating with a Zigbee NCP (Network Co-Processor) device.

use std::collections::BTreeMap;
use std::time::Duration;

use apis_saltans_core::Endpoint;
use macaddr::MacAddr8;
use tokio::sync::mpsc::Receiver;
use tokio::task::JoinHandle;

use self::sealed_driver::SealedDriver;
use crate::message::Message;
use crate::{Error, FoundNetwork, Frame, NcpHandle, ParallelUnicastResult, ScannedChannel};

mod sealed_driver;

/// A common Zigbee NCP driver interface.
pub trait NcpDriver {
    /// Get the next transaction sequence number.
    fn next_transaction_seq(&mut self) -> u8;

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
    fn get_ieee_address(&mut self) -> impl Future<Output = Result<MacAddr8, Error>> + Send;

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

    /// Get the list of neighbor devices.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn get_neighbors(
        &mut self,
    ) -> impl Future<Output = Result<BTreeMap<MacAddr8, u16>, Error>> + Send;

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
    ) -> impl Future<Output = Result<MacAddr8, Error>> + Send;

    /// Get the short ID of the device with the specified IEEE address.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn ieee_address_to_short_id(
        &mut self,
        ieee_address: MacAddr8,
    ) -> impl Future<Output = Result<u16, Error>> + Send;

    /// Send a unicast message.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn unicast(
        &mut self,
        address: u16,
        endpoint: Endpoint,
        frame: Frame,
    ) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Send a multicast message.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn multicast(
        &mut self,
        group_id: u16,
        hops: u8,
        radius: u8,
        frame: Frame,
    ) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Send a broadcast message.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn broadcast(
        &mut self,
        short_id: u16,
        radius: u8,
        frame: Frame,
    ) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Send multiple unicasts in parallel without waiting for the stack to confirm any sent frames.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn parallel_unicast(
        &mut self,
        targets: BTreeMap<u16, Box<[Endpoint]>>,
        frame: Frame,
    ) -> impl Future<Output = ParallelUnicastResult> + Send;

    /// Run the network manager actor.
    fn run(self, rx: Receiver<Message>) -> impl Future<Output = Self> + Send
    where
        Self: Sized + SealedDriver,
    {
        SealedDriver::run(self, rx)
    }

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
