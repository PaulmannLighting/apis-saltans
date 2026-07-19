use zb_core::Destination;
use zb_zcl::on_off::{Effect, Off, OffWithEffect, On, Toggle};

use crate::api::Zcl;
use crate::{Error, TransmissionResponse};

/// Trait for On/Off cluster operations.
///
/// Each method queues its command and returns a [`TransmissionResponse`]. Await the returned
/// response to observe the hardware transmission result.
pub trait OnOff {
    /// Turns the device on.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the command cannot be queued. The returned
    /// [`TransmissionResponse`] reports hardware transmission errors when awaited.
    fn on(
        &self,
        destination: Destination,
    ) -> impl Future<Output = Result<TransmissionResponse, Error>> + Send;

    /// Turns the device off.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the command cannot be queued. The returned
    /// [`TransmissionResponse`] reports hardware transmission errors when awaited.
    fn off(
        &self,
        destination: Destination,
    ) -> impl Future<Output = Result<TransmissionResponse, Error>> + Send;

    /// Turns the device off with the specified effect.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the command cannot be queued. The returned
    /// [`TransmissionResponse`] reports hardware transmission errors when awaited.
    fn off_with_effect(
        &self,
        destination: Destination,
        effect: Effect,
    ) -> impl Future<Output = Result<TransmissionResponse, Error>> + Send;

    /// Toggle the device state.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the command cannot be queued. The returned
    /// [`TransmissionResponse`] reports hardware transmission errors when awaited.
    fn toggle(
        &self,
        destination: Destination,
    ) -> impl Future<Output = Result<TransmissionResponse, Error>> + Send;
}

impl<T> OnOff for T
where
    T: Zcl + Sync,
{
    async fn on(&self, destination: Destination) -> Result<TransmissionResponse, Error> {
        self.transmit(destination, On).await
    }

    async fn off(&self, destination: Destination) -> Result<TransmissionResponse, Error> {
        self.transmit(destination, Off).await
    }

    async fn off_with_effect(
        &self,
        destination: Destination,
        effect: Effect,
    ) -> Result<TransmissionResponse, Error> {
        self.transmit(destination, OffWithEffect::new(effect)).await
    }

    async fn toggle(&self, destination: Destination) -> Result<TransmissionResponse, Error> {
        self.transmit(destination, Toggle).await
    }
}
