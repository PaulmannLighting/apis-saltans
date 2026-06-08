use zcl::general::on_off::{Off, On, Toggle};
use zigbee::{Address, Endpoint};

use crate::transceiver::zcl::Handle;
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

impl OnOff for Coordinator {
    async fn on(&self, address: Address, endpoint: Endpoint) -> Result<(), Error> {
        self.zcl_transceiver
            .unicast_zcl_native(address, endpoint, On)
            .await?;
        Ok(())
    }

    async fn off(&self, address: Address, endpoint: Endpoint) -> Result<(), Error> {
        self.zcl_transceiver
            .unicast_zcl_native(address, endpoint, Off)
            .await?;
        Ok(())
    }

    async fn toggle(&self, address: Address, endpoint: Endpoint) -> Result<(), Error> {
        self.zcl_transceiver
            .unicast_zcl_native(address, endpoint, Toggle)
            .await?;
        Ok(())
    }
}
