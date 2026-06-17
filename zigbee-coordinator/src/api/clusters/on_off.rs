use macaddr::MacAddr8;
use zcl::general::on_off::{Off, On, Toggle};
use zigbee::Endpoint;

use crate::transceiver::zcl::Handle;
use crate::{Coordinator, Error, NetworkManager};

/// Trait for On/Off cluster operations.
pub trait OnOff {
    /// Turns the device on.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn on(
        &self,
        ieee_address: MacAddr8,
        endpoint: Endpoint,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Turns the device off.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn off(
        &self,
        ieee_address: MacAddr8,
        endpoint: Endpoint,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Toggle the device state.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn toggle(
        &self,
        ieee_address: MacAddr8,
        endpoint: Endpoint,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl OnOff for Coordinator {
    async fn on(&self, ieee_address: MacAddr8, endpoint: Endpoint) -> Result<(), Error> {
        self.zcl
            .unicast_static_cluster(
                self.network_manager
                    .get_short_id_from_ieee_address(ieee_address)
                    .await?
                    .ok_or(Error::UnknownDevice(ieee_address))?,
                endpoint,
                On,
            )
            .await
    }

    async fn off(&self, ieee_address: MacAddr8, endpoint: Endpoint) -> Result<(), Error> {
        self.zcl
            .unicast_static_cluster(
                self.network_manager
                    .get_short_id_from_ieee_address(ieee_address)
                    .await?
                    .ok_or(Error::UnknownDevice(ieee_address))?,
                endpoint,
                Off,
            )
            .await
    }

    async fn toggle(&self, ieee_address: MacAddr8, endpoint: Endpoint) -> Result<(), Error> {
        self.zcl
            .unicast_static_cluster(
                self.network_manager
                    .get_short_id_from_ieee_address(ieee_address)
                    .await?
                    .ok_or(Error::UnknownDevice(ieee_address))?,
                endpoint,
                Toggle,
            )
            .await
    }
}
