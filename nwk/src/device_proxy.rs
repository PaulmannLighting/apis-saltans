use zcl::Commands;
use zigbee::Endpoint;

use crate::endpoint_proxy::EndpointProxy;
use crate::{Error, Proxy, ProxySender};

/// Extension trait to get a device proxy from a Network Layer Management Entity (NLME).
pub trait DeviceProxyExt: Sized {
    /// Get a device proxy for the specified PAN ID.
    fn device(&mut self, pan_id: u16) -> DeviceProxy<'_, Self>;
}

impl<T> DeviceProxyExt for ProxySender<T> {
    fn device(&mut self, pan_id: u16) -> DeviceProxy<'_, Self> {
        DeviceProxy::new(self, pan_id)
    }
}

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

impl<T> DeviceProxy<'_, ProxySender<T>> {
    /// Get a proxy for a specific endpoint on the device.
    pub const fn endpoint(&mut self, endpoint_id: Endpoint) -> EndpointProxy<'_, ProxySender<T>> {
        EndpointProxy::new(self.nlme, self.pan_id, endpoint_id)
    }

    /// Get a proxy for the default endpoint on the device.
    pub fn default_endpoint(&mut self) -> EndpointProxy<'_, ProxySender<T>> {
        self.endpoint(Endpoint::default())
    }
}

impl<T> DeviceProxy<'_, ProxySender<T>>
where
    T: std::error::Error,
{
    /// Send a unicast command to the device.
    pub async fn unicast_command(
        &mut self,
        endpoint: Endpoint,
        command: impl Into<Commands>,
    ) -> Result<(), Error<T>> {
        self.nlme
            .unicast_command(self.pan_id, endpoint, command)
            .await
    }
}
