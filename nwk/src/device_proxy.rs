use le_stream::ToLeStream;
use zcl::Command;

use crate::{Error, Nlme};

/// A proxy structure to interact with a Zigbee device via the Network Layer Management Entity (NLME).
#[derive(Debug)]
pub struct DeviceProxy<'nlme, T> {
    nlme: &'nlme mut T,
    pan_id: u16,
}

impl<'nlme, T> DeviceProxy<'nlme, T> {
    /// Create a new `DeviceProxy`.
    pub(crate) const fn new(nlme: &'nlme mut T, pan_id: u16) -> Self {
        Self { nlme, pan_id }
    }
}

impl<T> DeviceProxy<'_, T> {
    /// Send a unicast command to the device.
    pub async fn unicast_command<C>(
        &mut self,
        endpoint: u8,
        frame: C,
    ) -> Result<(), Error<T::Error>>
    where
        T: Nlme,
        C: Command + ToLeStream,
    {
        self.nlme
            .unicast_command(self.pan_id, endpoint, frame)
            .await
    }
}
