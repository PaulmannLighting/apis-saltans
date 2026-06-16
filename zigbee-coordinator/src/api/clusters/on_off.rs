use std::borrow::Borrow;

use tokio::sync::mpsc::Sender;
use zcl::general::on_off::{Off, On, Toggle};
use zigbee::{Address, Endpoint};

use crate::Error;
use crate::transceiver::zcl::{Handle, Message};

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

impl<T> OnOff for T
where
    T: Borrow<Sender<Message>> + Sync,
{
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
