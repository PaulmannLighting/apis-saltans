use std::collections::BTreeMap;
use std::time::Duration;

use le_stream::ToLeStream;
use macaddr::MacAddr8;

use crate::Error;
use crate::device_proxy::DeviceProxy;

/// Network layer management entity (NLME) trait.
pub trait Nlme {
    /// The error type returned by NLME operations.
    type Error: std::error::Error;

    /// Allow devices to join the network for the specified duration.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn allow_joins(
        &mut self,
        duration: Duration,
    ) -> impl Future<Output = Result<(), Error<Self::Error>>>;

    /// Get the list of neighbor devices.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn get_neighbors(
        &mut self,
    ) -> impl Future<Output = Result<BTreeMap<MacAddr8, Option<u16>>, Error<Self::Error>>>;

    /// Send a unicast message.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn unicast_command<T>(
        &mut self,
        pan_id: u16,
        endpoint: u8,
        frame: T,
    ) -> impl Future<Output = Result<(), Error<Self::Error>>>
    where
        T: zcl::Command + ToLeStream;

    /// Return a proxy for the device with the specified PAN ID.
    fn device(&mut self, pan_id: u16) -> DeviceProxy<'_, Self>
    where
        Self: Sized,
    {
        DeviceProxy::new(self, pan_id)
    }
}
