use le_stream::ToLeStream;
use zcl::Command;
use zdp::Service;
use zigbee::{Cluster, ClusterId, Endpoint};

use crate::{Error, Frame, Proxy};

/// A proxy for an endpoint within a network layer management entity (NLME).
#[derive(Debug)]
pub struct EndpointProxy<'proxy, T> {
    proxy: &'proxy T,
    pan_id: u16,
    endpoint: Endpoint,
}

impl<'proxy, T> EndpointProxy<'proxy, T> {
    /// Create a new `EndpointProxy`.
    pub(crate) const fn new(proxy: &'proxy T, pan_id: u16, endpoint: Endpoint) -> Self {
        Self {
            proxy,
            pan_id,
            endpoint,
        }
    }
}

impl<T> EndpointProxy<'_, T>
where
    T: Proxy + Sync,
{
    /// Send a unicast command to the endpoint.
    pub async fn unicast(&self, frame: Frame) -> Result<u8, Error> {
        self.proxy.unicast(self.pan_id, self.endpoint, frame).await
    }

    /// Send a unicast ZCL command to the endpoint.
    pub async fn unicast_zcl<C>(&self, command: C) -> Result<u8, Error>
    where
        C: Command + ClusterId + ToLeStream,
    {
        self.proxy
            .zcl()
            .unicast(self.pan_id, self.endpoint, command)
            .await
    }

    /// Send a unicast ZCL command to the endpoint.
    pub async fn unicast_zdp<C>(&self, command: C) -> Result<u8, Error>
    where
        C: Cluster + Service + ToLeStream,
    {
        self.proxy
            .zdp()
            .unicast(self.pan_id, self.endpoint, command)
            .await
    }
}
