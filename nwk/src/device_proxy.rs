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
