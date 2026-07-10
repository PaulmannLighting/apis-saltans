use std::collections::BTreeMap;

use zb_core::node::Descriptor;
use zb_core::{Endpoint, FullAddress, IeeeAddress};
use zb_zdp::SimpleDescriptor;

/// Type alias for a map of devices to their endpoints.
pub type Devices = BTreeMap<IeeeAddress, OutgoingDevice>;

#[derive(Debug)]
pub struct OutgoingDevice {
    pub(crate) address: FullAddress,
    pub(crate) descriptor: Descriptor,
    pub(crate) endpoints: BTreeMap<Endpoint, Option<SimpleDescriptor>>,
}

impl From<super::IncomingDevice> for OutgoingDevice {
    fn from(device: super::IncomingDevice) -> Self {
        Self {
            address: device.address,
            descriptor: device.descriptor,
            endpoints: device
                .endpoints
                .into_iter()
                .map(|endpoint| (endpoint, None))
                .collect(),
        }
    }
}
