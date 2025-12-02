use std::error::Error;

use crate::NetworkDescriptor;

/// Network layer management entity (NLME) trait.
pub trait Nlme {
    /// Device settings type.
    type DeviceSettings;
    /// Error type.
    type Error: Error;

    /// Configure a device on the network.
    fn configure(
        &mut self,
        settings: Self::DeviceSettings,
    ) -> impl Future<Output = Result<(), Self::Error>>;

    /// Start the network.
    fn start(&mut self, reinitialize: bool) -> impl Future<Output = Result<(), Self::Error>>;

    /// Join a network.
    fn join(
        &mut self,
        settings: Self::DeviceSettings,
    ) -> impl Future<Output = Result<(), Self::Error>>;

    /// Rejoin a network.
    fn rejoin(
        &mut self,
        network: NetworkDescriptor,
    ) -> impl Future<Output = Result<(), Self::Error>>;

    /// Leave the network.
    fn leave(&mut self) -> impl Future<Output = Result<(), Self::Error>>;

    // TODO: Add more NLME methods as needed. Maybe split into separate traits.
}
