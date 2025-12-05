use le_stream::ToLeStream;
use zigbee::{Command, Endpoint};

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
    pub async fn unicast(&self, cluster_id: u16, group_id: u16, frame: Frame) -> Result<(), Error> {
        self.proxy
            .unicast(self.pan_id, self.endpoint, cluster_id, group_id, frame)
            .await
    }

    /// Send a unicast ZCL command to the endpoint.
    pub async fn unicast_zcl<C>(&self, group_id: u16, command: C) -> Result<(), Error>
    where
        C: Command + ToLeStream,
    {
        self.proxy
            .zcl()
            .unicast(self.pan_id, self.endpoint, group_id, command)
            .await
    }
}
