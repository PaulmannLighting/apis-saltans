use zcl::general::on_off::{Off, On, Toggle};
use zigbee::{Address, Endpoint};
use zigbee_hw::Error;

use crate::Coordinator;
use crate::transmitter::Handle;

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

impl OnOff for Coordinator {
    async fn on(&self, address: Address, endpoint: Endpoint) -> Result<(), Error> {
        self.transmitter
            .unicast_zcl_native(address, endpoint, On)
            .await
    }

    async fn off(&self, address: Address, endpoint: Endpoint) -> Result<(), Error> {
        self.transmitter
            .unicast_zcl_native(address, endpoint, Off)
            .await
    }

    async fn toggle(&self, address: Address, endpoint: Endpoint) -> Result<(), Error> {
        self.transmitter
            .unicast_zcl_native(address, endpoint, Toggle)
            .await
    }
}
