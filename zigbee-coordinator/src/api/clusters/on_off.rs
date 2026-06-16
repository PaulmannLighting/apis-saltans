use tokio::sync::mpsc::Sender;
use zcl::general::on_off::{Off, On, Toggle};
use zigbee::{Address, Endpoint};

use crate::transceiver::zcl::{Handle, Message};
use crate::{Coordinator, Error};

/// Trait for On/Off cluster operations.
pub trait OnOff {
    /// Turns the device on.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn on(
        &self,
        address: Address,
        endpoint: Endpoint,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Turns the device off.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn off(
        &self,
        address: Address,
        endpoint: Endpoint,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Toggle the device state.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn toggle(
        &self,
        address: Address,
        endpoint: Endpoint,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl OnOff for Sender<Message> {
    async fn on(&self, address: Address, endpoint: Endpoint) -> Result<(), Error> {
        self.unicast_zcl_native(address.short_id(), endpoint, On)
            .await
    }

    async fn off(&self, address: Address, endpoint: Endpoint) -> Result<(), Error> {
        self.unicast_zcl_native(address.short_id(), endpoint, Off)
            .await
    }

    async fn toggle(&self, address: Address, endpoint: Endpoint) -> Result<(), Error> {
        self.unicast_zcl_native(address.short_id(), endpoint, Toggle)
            .await
    }
}

impl OnOff for Coordinator {
    async fn on(&self, address: Address, endpoint: Endpoint) -> Result<(), Error> {
        self.zcl.on(address, endpoint).await
    }

    async fn off(&self, address: Address, endpoint: Endpoint) -> Result<(), Error> {
        self.zcl.off(address, endpoint).await
    }

    async fn toggle(&self, address: Address, endpoint: Endpoint) -> Result<(), Error> {
        self.zcl.toggle(address, endpoint).await
    }
}
