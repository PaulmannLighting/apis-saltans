use std::collections::BTreeMap;
use std::time::Duration;

use le_stream::ToLeStream;
use macaddr::MacAddr8;
use zigbee::Endpoint;

use crate::Error;

/// Network layer management entity (NLME) trait.
pub trait Nlme {
    /// Get the PAN ID of the network manager.
    fn get_pan_id(&self) -> impl Future<Output = Result<u16, Error>>;

    /// Allow devices to join the network for the specified duration.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn allow_joins(&mut self, duration: Duration) -> impl Future<Output = Result<(), Error>>;

    /// Get the list of neighbor devices.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn get_neighbors(&mut self) -> impl Future<Output = Result<BTreeMap<MacAddr8, u16>, Error>>;

    /// Send a unicast message.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn unicast_command<T>(
        &mut self,
        pan_id: u16,
        endpoint: Endpoint,
        frame: T,
    ) -> impl Future<Output = Result<(), Error>>
    where
        T: zcl::Command + ToLeStream;
}
