/// A proxy for an endpoint within a network layer management entity (NLME).
#[derive(Debug)]
pub struct EndpointProxy<'nlme, T> {
    nlme: &'nlme mut T,
    pan_id: u16,
    endpoint_id: u8,
}

impl<'nlme, T> EndpointProxy<'nlme, T> {
    /// Create a new `EndpointProxy`.
    pub(crate) const fn new(nlme: &'nlme mut T, pan_id: u16, endpoint_id: u8) -> Self {
        Self {
            nlme,
            pan_id,
            endpoint_id,
        }
    }
}
