use zcl::Commands;
use zigbee::Endpoint;

use crate::{Error, Proxy};

/// A proxy for an endpoint within a network layer management entity (NLME).
#[derive(Debug)]
pub struct EndpointProxy<'proxy, T> {
    proxy: &'proxy mut T,
    pan_id: u16,
    endpoint_id: Endpoint,
}

impl<'proxy, T> EndpointProxy<'proxy, T> {
    /// Create a new `EndpointProxy`.
    pub(crate) const fn new(proxy: &'proxy mut T, pan_id: u16, endpoint_id: Endpoint) -> Self {
        Self {
            proxy,
            pan_id,
            endpoint_id,
        }
    }
}

impl<T> EndpointProxy<'_, T>
where
    T: Proxy,
{
    /// Send a unicast command to the endpoint.
    pub async fn unicast_command(&mut self, command: impl Into<Commands>) -> Result<(), Error> {
        self.proxy
            .unicast_command(self.pan_id, self.endpoint_id, command)
            .await
    }
}
