use std::collections::BTreeMap;
use std::time::Duration;

use apis_saltans_core::{Address, Endpoint};
use macaddr::MacAddr8;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::channel;

use crate::message::Message;
use crate::{Error, FoundNetwork, Frame, ScannedChannel};

/// Proxy trait to communicate with Zigbee NCPs which implement [`NcpDriver`](crate::NcpDriver).
///
/// This trait is implemented for `Sender<Message>`, allowing you to communicate with a Zigbee NCP.
pub trait Ncp {
    /// Get the next transaction sequence number.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn next_transaction_seq(&self) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Get the short ID of the network manager.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn get_pan_id(&self) -> impl Future<Output = Result<u16, Error>> + Send;

    /// Get the IEEE address of the network manager.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn get_ieee_address(&self) -> impl Future<Output = Result<MacAddr8, Error>> + Send;

    /// Return the full address of the coordinator.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn get_address(&self) -> impl Future<Output = Result<Address, Error>> + Send
    where
        Self: Sync,
    {
        async {
            Ok(Address::new(
                self.get_ieee_address().await?,
                self.get_pan_id().await?,
            ))
        }
    }

    /// Scan for available networks.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn scan_networks(
        &self,
        channel_mask: u32,
        duration: u8,
    ) -> impl Future<Output = Result<Vec<FoundNetwork>, Error>> + Send;

    /// Scan channels for activity.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn scan_channels(
        &self,
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
        &self,
        duration: Duration,
    ) -> impl Future<Output = Result<Duration, Error>> + Send;

    /// Get the list of neighbor devices.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn get_neighbors(&self) -> impl Future<Output = Result<BTreeMap<MacAddr8, u16>, Error>> + Send;

    /// Send a route request with the specified radius.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn route_request(&self, radius: u8) -> impl Future<Output = Result<(), Error>> + Send;

    /// Get the IEEE address of the device with the specified short ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn short_id_to_ieee_address(
        &self,
        short_id: u16,
    ) -> impl Future<Output = Result<MacAddr8, Error>> + Send;

    /// Get the short ID of the device with the specified IEEE address.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn ieee_address_to_short_id(
        &self,
        ieee_address: MacAddr8,
    ) -> impl Future<Output = Result<u16, Error>> + Send;

    /// Send a unicast ZCL command.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn unicast(
        &self,
        short_id: u16,
        endpoint: Endpoint,
        frame: Frame,
    ) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Send a multicast ZCL command.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn multicast(
        &self,
        group_id: u16,
        hops: u8,
        radius: u8,
        frame: Frame,
    ) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Send a broadcast ZCL command.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn broadcast(
        &self,
        short_id: u16,
        radius: u8,
        frame: Frame,
    ) -> impl Future<Output = Result<u8, Error>> + Send;
}

impl Ncp for Sender<Message> {
    async fn next_transaction_seq(&self) -> Result<u8, Error> {
        let (response, rx) = channel();
        self.send(Message::GetTransactionSeq { response }).await?;
        Ok(rx.await?)
    }

    async fn get_pan_id(&self) -> Result<u16, Error> {
        let (response, rx) = channel();
        self.send(Message::GetPanId { response }).await?;
        rx.await?
    }

    async fn get_ieee_address(&self) -> Result<MacAddr8, Error> {
        let (response, rx) = channel();
        self.send(Message::GetIeeeAddress { response }).await?;
        rx.await?
    }

    async fn scan_networks(
        &self,
        channel_mask: u32,
        duration: u8,
    ) -> Result<Vec<FoundNetwork>, Error> {
        let (response, rx) = channel();
        self.send(Message::ScanNetworks {
            channel_mask,
            duration,
            response,
        })
        .await?;
        rx.await?
    }

    async fn scan_channels(
        &self,
        channel_mask: u32,
        duration: u8,
    ) -> Result<Vec<ScannedChannel>, Error> {
        let (response, rx) = channel();
        self.send(Message::ScanChannels {
            channel_mask,
            duration,
            response,
        })
        .await?;
        rx.await?
    }

    async fn allow_joins(&self, duration: Duration) -> Result<Duration, Error> {
        let (response, rx) = channel();
        self.send(Message::AllowJoins { duration, response })
            .await?;
        rx.await?
    }

    async fn get_neighbors(&self) -> Result<BTreeMap<MacAddr8, u16>, Error> {
        let (response, rx) = channel();
        self.send(Message::GetNeighbors { response }).await?;
        rx.await?
    }

    async fn route_request(&self, radius: u8) -> Result<(), Error> {
        let (response, rx) = channel();
        self.send(Message::RouteRequest { radius, response })
            .await?;
        rx.await?
    }

    async fn short_id_to_ieee_address(&self, short_id: u16) -> Result<MacAddr8, Error> {
        let (response, rx) = channel();
        self.send(Message::TranslateIeeeAddress { short_id, response })
            .await?;
        rx.await?
    }

    async fn ieee_address_to_short_id(&self, ieee_address: MacAddr8) -> Result<u16, Error> {
        let (response, rx) = channel();
        self.send(Message::TranslateShortId {
            ieee_address,
            response,
        })
        .await?;
        rx.await?
    }

    async fn unicast(&self, short_id: u16, endpoint: Endpoint, frame: Frame) -> Result<u8, Error> {
        let (response, rx) = channel();
        self.send(Message::Unicast {
            short_id,
            endpoint,
            frame,
            response,
        })
        .await?;
        rx.await?
    }

    async fn multicast(
        &self,
        group_id: u16,
        hops: u8,
        radius: u8,
        frame: Frame,
    ) -> Result<u8, Error> {
        let (response, rx) = channel();
        self.send(Message::Multicast {
            group_id,
            hops,
            radius,
            frame,
            response,
        })
        .await?;
        rx.await?
    }

    async fn broadcast(&self, short_id: u16, radius: u8, frame: Frame) -> Result<u8, Error> {
        let (response, rx) = channel();
        self.send(Message::Broadcast {
            short_id,
            radius,
            frame,
            response,
        })
        .await?;
        rx.await?
    }
}
