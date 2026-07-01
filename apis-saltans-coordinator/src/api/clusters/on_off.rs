use std::collections::BTreeMap;

use apis_saltans_core::Endpoint;
use apis_saltans_zcl::general::on_off::{Effect, Off, OffWithEffect, On, Toggle};

use crate::transceiver::zcl::Handle;
use crate::{Coordinator, Destination, Error};

type ParallelUnicastResult = Result<BTreeMap<(u16, Endpoint), Result<u8, Error>>, Error>;

/// Trait for On/Off cluster operations.
pub trait OnOff {
    /// Turns the device on.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn on(&self, destination: Destination) -> impl Future<Output = Result<(), Error>> + Send;

    /// Turns all devices on in parallel.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn all_on(
        &self,
        destinations: Box<[Destination]>,
    ) -> impl Future<Output = ParallelUnicastResult> + Send;

    /// Turns the device off.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn off(&self, destination: Destination) -> impl Future<Output = Result<(), Error>> + Send;

    /// Turns all device off in parallel.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn all_off(
        &self,
        destinations: Box<[Destination]>,
    ) -> impl Future<Output = ParallelUnicastResult> + Send;

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
        self.send_static_cluster(destination, On).await
    }

    async fn all_on(&self, destinations: Box<[Destination]>) -> ParallelUnicastResult {
        self.send_static_cluster_parallel(destinations, On).await
    }

    async fn off(&self, destination: Destination) -> Result<(), Error> {
        self.send_static_cluster(destination, Off).await
    }

    async fn all_off(&self, destinations: Box<[Destination]>) -> ParallelUnicastResult {
        self.send_static_cluster_parallel(destinations, Off).await
    }

    async fn off_with_effect(&self, destination: Destination, effect: Effect) -> Result<(), Error> {
        self.send_static_cluster(destination, OffWithEffect::new(effect))
            .await
    }

    async fn toggle(&self, destination: Destination) -> Result<(), Error> {
        self.send_static_cluster(destination, Toggle).await
    }
}
