#![cfg(feature = "coordinator")]

//! Coordinator-facing hardware abstraction API.

use std::time::Duration;

use apis_saltans_core::{Address, Destination, IeeeAddress};
use tokio::sync::oneshot::channel;

use crate::common::{Datagram, FoundNetwork, Message, ScannedChannel};
use crate::{Error, NcpHandle};

/// Proxy trait for sending commands to a Zigbee NCP driver actor.
pub trait Ncp {
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
    fn get_ieee_address(&self) -> impl Future<Output = Result<IeeeAddress, Error>> + Send;

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
    ) -> impl Future<Output = Result<IeeeAddress, Error>> + Send;

    /// Get the short ID of the device with the specified IEEE address.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn ieee_address_to_short_id(
        &self,
        ieee_address: IeeeAddress,
    ) -> impl Future<Output = Result<u16, Error>> + Send;

    /// Transmit a serialized application datagram to a destination.
    ///
    /// # Errors
    ///
    /// Returns an error if the actor is unavailable or the driver fails to transmit the datagram.
    fn transmit(
        &self,
        destination: Destination,
        datagram: Datagram,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl Ncp for NcpHandle {
    async fn get_pan_id(&self) -> Result<u16, Error> {
        let (response, rx) = channel();
        self.send(Message::GetPanId { response }).await?;
        rx.await?
    }

    async fn get_ieee_address(&self) -> Result<IeeeAddress, Error> {
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

    async fn route_request(&self, radius: u8) -> Result<(), Error> {
        let (response, rx) = channel();
        self.send(Message::RouteRequest { radius, response })
            .await?;
        rx.await?
    }

    async fn short_id_to_ieee_address(&self, short_id: u16) -> Result<IeeeAddress, Error> {
        let (response, rx) = channel();
        self.send(Message::TranslateIeeeAddress { short_id, response })
            .await?;
        rx.await?
    }

    async fn ieee_address_to_short_id(&self, ieee_address: IeeeAddress) -> Result<u16, Error> {
        let (response, rx) = channel();
        self.send(Message::TranslateShortId {
            ieee_address,
            response,
        })
        .await?;
        rx.await?
    }

    async fn transmit(&self, destination: Destination, datagram: Datagram) -> Result<(), Error> {
        let (response, rx) = channel();
        self.send(Message::Transmit {
            destination,
            datagram,
            response,
        })
        .await?;
        rx.await?
    }
}
