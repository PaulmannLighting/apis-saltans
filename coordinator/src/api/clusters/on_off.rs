use zb_core::Destination;
use zb_zcl::on_off::{Effect, Off, OffWithEffect, On, Toggle};

use crate::transceiver::zcl::Handle;
use crate::{Coordinator, Error};

/// Trait for On/Off cluster operations.
pub trait OnOff {
    /// Turns the device on.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn on(&self, destination: Destination) -> impl Future<Output = Result<(), Error>> + Send;

    /// Turns the device off.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn off(&self, destination: Destination) -> impl Future<Output = Result<(), Error>> + Send;

    /// Turns the device off with the specified effect.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn off_with_effect(
        &self,
        destination: Destination,
        effect: Effect,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Toggle the device state.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn toggle(&self, destination: Destination) -> impl Future<Output = Result<(), Error>> + Send;
}

impl OnOff for Coordinator {
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
