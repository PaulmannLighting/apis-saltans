use std::collections::BTreeMap;
use std::time::Duration;

use macaddr::MacAddr8;
use tokio::sync::mpsc::Receiver;
use zigbee::Endpoint;

use crate::message::Message;
use crate::{Error, FoundNetwork, Frame, ScannedChannel};

mod sealed;

/// A Zigbee network manager.
pub trait Actor {
    /// Get the next transaction sequence number.
    fn next_transaction_seq(&mut self) -> u8;

    /// Get the PAN ID of the network manager.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn get_pan_id(&mut self) -> impl Future<Output = Result<u16, Error>> + Send;

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
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn allow_joins(&mut self, duration: Duration)
    -> impl Future<Output = Result<(), Error>> + Send;

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

    /// Get the IEEE address of the device with the specified PAN ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn get_ieee_address(
        &mut self,
        pan_id: u16,
    ) -> impl Future<Output = Result<MacAddr8, Error>> + Send;

    /// Send a unicast message.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn unicast(
        &mut self,
        pan_id: u16,
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
        pan_id: u16,
        radius: u8,
        frame: Frame,
    ) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Run the network manager actor.
    fn run(self, rx: Receiver<Message>) -> impl Future<Output = ()> + Send
    where
        Self: Sized + sealed::Actor,
    {
        sealed::Actor::run(self, rx)
    }
}
