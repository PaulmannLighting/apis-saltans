use le_stream::ToLeStream;
use zcl::Command;
use zigbee::Endpoint;

use crate::endpoint_proxy::EndpointProxy;
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

impl<T> DeviceProxy<'_, T>
where
    T: Nlme,
{
    /// Get a proxy for a specific endpoint on the device.
    pub const fn endpoint(&mut self, endpoint_id: Endpoint) -> EndpointProxy<'_, T> {
        EndpointProxy::new(self.nlme, self.pan_id, endpoint_id)
    }

    /// Get a proxy for the default endpoint on the device.
    pub fn default_endpoint(&mut self) -> EndpointProxy<'_, T> {
        self.endpoint(Endpoint::default())
    }
}

impl<T> DeviceProxy<'_, T>
where
    T: Nlme,
{
    /// Send a unicast command to the device.
    pub async fn unicast_command<C>(
        &mut self,
        endpoint: Endpoint,
        frame: C,
    ) -> Result<(), Error<T::Error>>
    where
        C: Command + ToLeStream,
    {
        self.nlme
            .unicast_command(self.pan_id, endpoint, frame)
            .await
    }
}
