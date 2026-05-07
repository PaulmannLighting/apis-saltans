use zcl::HeaderFactory;
use zcl::general::on_off::{Off, On, Toggle};
use zigbee::Endpoint;

use crate::Error;
use crate::zcl::tx_rx::Transmitter;

/// Trait for On/Off cluster operations.
pub trait OnOff {
    /// Turns the device on.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn on(&self, pan_id: u16, endpoint: Endpoint)
    -> impl Future<Output = Result<u8, Error>> + Send;

    /// Turns the device off.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn off(
        &self,
        pan_id: u16,
        endpoint: Endpoint,
    ) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Toggle the device state.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn toggle(
        &self,
        pan_id: u16,
        endpoint: Endpoint,
    ) -> impl Future<Output = Result<u8, Error>> + Send;
}

impl<T> OnOff for T
where
    T: Transmitter + Sync,
{
    async fn on(&self, pan_id: u16, endpoint: Endpoint) -> Result<u8, Error> {
        self.send(pan_id, endpoint, On.frame(self.next_seq().await?))
            .await
    }

    async fn off(&self, pan_id: u16, endpoint: Endpoint) -> Result<u8, Error> {
        self.send(pan_id, endpoint, Off.frame(self.next_seq().await?))
            .await
    }

    async fn toggle(&self, pan_id: u16, endpoint: Endpoint) -> Result<u8, Error> {
        self.send(pan_id, endpoint, Toggle.frame(self.next_seq().await?))
            .await
    }
}
