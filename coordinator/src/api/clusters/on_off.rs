use zb_core::Destination;
use zb_zcl::on_off::{Effect, Off, OffWithEffect, On, Toggle};

use crate::Error;
use crate::api::Zcl;

/// Trait for On/Off cluster operations.
///
/// Each method awaits the acknowledged APS transmission before returning.
pub trait OnOff {
    /// Turns the device on.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the command cannot be queued or transmitted.
    fn on(&self, destination: Destination) -> impl Future<Output = Result<(), Error>> + Send;

    /// Turns the device off.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the command cannot be queued or transmitted.
    fn off(&self, destination: Destination) -> impl Future<Output = Result<(), Error>> + Send;

    /// Turns the device off with the specified effect.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the command cannot be queued or transmitted.
    fn off_with_effect(
        &self,
        destination: Destination,
        effect: Effect,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Toggle the device state.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the command cannot be queued or transmitted.
    fn toggle(&self, destination: Destination) -> impl Future<Output = Result<(), Error>> + Send;
}

impl<T> OnOff for T
where
    T: Zcl + Sync,
{
    async fn on(&self, destination: Destination) -> Result<(), Error> {
        self.transmit(destination, On).await
    }

    async fn off(&self, destination: Destination) -> Result<(), Error> {
        self.transmit(destination, Off).await
    }

    async fn off_with_effect(&self, destination: Destination, effect: Effect) -> Result<(), Error> {
        self.transmit(destination, OffWithEffect::new(effect)).await
    }

    async fn toggle(&self, destination: Destination) -> Result<(), Error> {
        self.transmit(destination, Toggle).await
    }
}
