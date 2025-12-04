use le_stream::ToLeStream;
use zcl::Command;
use zigbee::Endpoint;

use crate::{Error, Nlme};

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

impl<T> EndpointProxy<'_, T>
where
    T: Nlme,
{
    /// Send a unicast command to the endpoint.
    pub async fn unicast_command<C>(&mut self, frame: C) -> Result<(), Error<T::Error>>
    where
        C: Command + ToLeStream,
    {
        self.nlme
            .unicast_command(self.pan_id, self.endpoint_id, frame)
            .await
    }
}
