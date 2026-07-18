use std::collections::BTreeMap;
use std::time::Duration;

use tokio::sync::mpsc::{Receiver, channel};
use zb_aps::data::Header;
use zb_core::{Application, Destination, IeeeAddress};
use zb_nwk::Metadata;

use crate::common::Message;
use crate::{Clusters, Datagram, Error, FoundNetwork, NcpHandle, ScannedChannel};

/// A common Zigbee NCP driver interface.
pub trait Driver {
    /// Return the local endpoint cluster sets registered with the NCP.
    ///
    /// The returned map is keyed by application endpoint ID. Driver implementations should report
    /// the input and output clusters that each local coordinator endpoint exposes to the Zigbee
    /// network.
    ///
    /// # Errors
    ///
    /// Returns an error if endpoint cluster information cannot be read or is not available.
    fn get_endpoints(
        &self,
    ) -> impl Future<Output = Result<BTreeMap<Application, Clusters>, Error>> + Send;

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

    fn send_reply(
        &mut self,
        node_id: u16,
        aps_header: Header,
        metadata: Metadata,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Spawn the actor in a tokio task.
    ///
    /// # Returns
    ///
    /// Returns a tuple of the tokio task's join handle and an actor proxy.
    fn run(self, channel_size: usize) -> (NcpHandle, impl Future<Output = Self> + Send)
    where
        Self: Sized + SealedDriver + 'static,
    {
        SealedDriver::spawn(self, channel_size)
    }
}

/// Sealed driver trait for handling actor communication with the Zigbee NCP.
///
/// This trait should not be implemented directly. Instead, implement the [`Driver`] trait for your
/// NCP type, and this `SealedDriver` trait will be automatically implemented for it.
pub trait SealedDriver {
    /// Run the actor, processing incoming messages.
    fn run(self, rx: Receiver<Message>) -> impl Future<Output = Self> + Send;

    /// Spawn the actor in a new tokio task.
    ///
    /// # Returns
    ///
    /// Returns a tuple of the tokio task's join handle and an actor proxy.
    fn spawn(self, channel_size: usize) -> (NcpHandle, impl Future<Output = Self> + Send)
    where
        Self: Sized + 'static;
}

impl<T> SealedDriver for T
where
    T: Driver + Send + 'static,
{
    async fn run(mut self, mut rx: Receiver<Message>) -> Self {
        while let Some(message) = rx.recv().await {
            match message {
                Message::GetEndpoints { response } => {
                    response
                        .send(self.get_endpoints().await)
                        .unwrap_or_else(drop);
                }
                Message::GetPanId { response } => {
                    response.send(self.get_pan_id().await).unwrap_or_else(drop);
                }
                Message::GetIeeeAddress { response } => {
                    response
                        .send(self.get_ieee_address().await)
                        .unwrap_or_else(drop);
                }
                Message::ScanNetworks {
                    channel_mask,
                    duration,
                    response,
                } => {
                    response
                        .send(self.scan_networks(channel_mask, duration).await)
                        .unwrap_or_else(drop);
                }
                Message::ScanChannels {
                    channel_mask,
                    duration,
                    response,
                } => {
                    response
                        .send(self.scan_channels(channel_mask, duration).await)
                        .unwrap_or_else(drop);
                }
                Message::AllowJoins { duration, response } => {
                    response
                        .send(self.allow_joins(duration).await)
                        .unwrap_or_else(drop);
                }
                Message::RouteRequest { radius, response } => {
                    response
                        .send(self.route_request(radius).await)
                        .unwrap_or_else(drop);
                }
                Message::TranslateIeeeAddress { short_id, response } => {
                    response
                        .send(self.short_id_to_ieee_address(short_id).await)
                        .unwrap_or_else(drop);
                }
                Message::TranslateShortId {
                    ieee_address,
                    response,
                } => {
                    response
                        .send(self.ieee_address_to_short_id(ieee_address).await)
                        .unwrap_or_else(drop);
                }
                Message::Transmit {
                    destination,
                    datagram,
                    response,
                } => {
                    response
                        .send(self.transmit(destination, datagram).await)
                        .unwrap_or_else(drop);
                }
                Message::SendReply {
                    node_id,
                    aps_header,
                    metadata,
                    response,
                } => response
                    .send(self.send_reply(node_id, aps_header, metadata).await)
                    .unwrap_or_else(drop),
            }
        }

        self
    }

    fn spawn(self, channel_size: usize) -> (NcpHandle, impl Future<Output = Self> + Send)
    where
        Self: 'static,
    {
        let (tx, rx) = channel(channel_size);
        let future = SealedDriver::run(self, rx);
        (tx, future)
    }
}
