use zigbee::Endpoint;

use crate::{Error, Proxy, ProxySender, ZclCommand};

/// A proxy for an endpoint within a network layer management entity (NLME).
#[derive(Debug)]
pub struct EndpointProxy<'nlme, T> {
    nlme: &'nlme mut T,
    pan_id: u16,
    endpoint_id: Endpoint,
}

impl<'nlme, T> EndpointProxy<'nlme, T> {
    /// Create a new `EndpointProxy`.
    pub(crate) const fn new(nlme: &'nlme mut T, pan_id: u16, endpoint_id: Endpoint) -> Self {
        Self {
            nlme,
            pan_id,
            endpoint_id,
        }
    }
}

impl<T> EndpointProxy<'_, ProxySender<T>>
where
    T: std::error::Error,
{
    /// Send a unicast command to the endpoint.
    pub async fn unicast_command(
        &mut self,
        command: impl Into<ZclCommand>,
    ) -> Result<(), Error<T>> {
        self.nlme
            .unicast_command(self.pan_id, self.endpoint_id, command)
            .await
    }
}
