use le_stream::ToLeStream;
use zcl::Command;
use zdp::Service;
use zigbee::{Cluster, Endpoint};

use crate::endpoint_proxy::EndpointProxy;
use crate::{Error, Frame, Proxy};

/// A proxy structure to interact with a Zigbee device via the Network Layer Management Entity (NLME).
#[derive(Debug)]
pub struct DeviceProxy<'proxy, T> {
    proxy: &'proxy T,
    pan_id: u16,
}

impl<'proxy, T> DeviceProxy<'proxy, T> {
    /// Create a new `DeviceProxy`.
    pub(crate) const fn new(proxy: &'proxy T, pan_id: u16) -> Self {
        Self { proxy, pan_id }
    }
}

impl<T> DeviceProxy<'_, T>
where
    T: Proxy + Sync,
{
    /// Get a proxy for a specific endpoint on the device.
    pub const fn endpoint(&self, endpoint_id: Endpoint) -> EndpointProxy<'_, T> {
        EndpointProxy::new(self.proxy, self.pan_id, endpoint_id)
    }

    /// Get a proxy for the default endpoint on the device.
    pub fn default_endpoint(&self) -> EndpointProxy<'_, T> {
        self.endpoint(Endpoint::default())
    }

    /// Send a unicast command to the device.
    pub async fn unicast_command(&self, endpoint: Endpoint, frame: Frame) -> Result<(), Error> {
        self.proxy.unicast(self.pan_id, endpoint, frame).await
    }

    /// Send a unicast ZCL command to the device.
    pub async fn unicast_zcl<C>(&self, endpoint: Endpoint, command: C) -> Result<(), Error>
    where
        C: Command + ToLeStream,
    {
        self.proxy
            .zcl()
            .unicast(self.pan_id, endpoint, command)
            .await
    }

    /// Send a unicast ZCL command to the device.
    pub async fn unicast_zdp<C>(&self, endpoint: Endpoint, command: C) -> Result<(), Error>
    where
        C: Cluster + Service + ToLeStream,
    {
        self.proxy
            .zdp()
            .unicast(self.pan_id, endpoint, command)
            .await
    }
}
