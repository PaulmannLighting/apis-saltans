use zcl::Commands;
use zigbee::Endpoint;

use crate::endpoint_proxy::EndpointProxy;
use crate::{Error, Proxy};

/// A proxy structure to interact with a Zigbee device via the Network Layer Management Entity (NLME).
#[derive(Debug)]
pub struct DeviceProxy<'proxy, T> {
    proxy: &'proxy mut T,
    pan_id: u16,
}

impl<'proxy, T> DeviceProxy<'proxy, T> {
    /// Create a new `DeviceProxy`.
    pub(crate) const fn new(proxy: &'proxy mut T, pan_id: u16) -> Self {
        Self { proxy, pan_id }
    }
}

impl<T> DeviceProxy<'_, T>
where
    T: Proxy,
{
    /// Get a proxy for a specific endpoint on the device.
    pub const fn endpoint(&mut self, endpoint_id: Endpoint) -> EndpointProxy<'_, T> {
        EndpointProxy::new(self.proxy, self.pan_id, endpoint_id)
    }

    /// Get a proxy for the default endpoint on the device.
    pub fn default_endpoint(&mut self) -> EndpointProxy<'_, T> {
        self.endpoint(Endpoint::default())
    }

    /// Send a unicast command to the device.
    pub async fn unicast_command(
        &mut self,
        endpoint: Endpoint,
        command: impl Into<Commands>,
    ) -> Result<(), Error> {
        self.proxy
            .unicast_command(self.pan_id, endpoint, command)
            .await
    }
}
