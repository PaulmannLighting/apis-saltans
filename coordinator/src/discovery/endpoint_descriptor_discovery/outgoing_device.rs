use std::collections::BTreeMap;

use apis_saltans_core::node::Descriptor;
use apis_saltans_core::{Endpoint, FullAddress, IeeeAddress};
use apis_saltans_zdp::SimpleDescriptor;

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
