use std::collections::BTreeMap;

use apis_saltans_core::node::Descriptor;
use apis_saltans_core::{Address, Endpoint};
use apis_saltans_zdp::SimpleDescriptor;

/// Type alias for a map of devices to their endpoints.
pub type Devices = BTreeMap<Address, Device>;

#[derive(Debug)]
pub struct Device {
    pub(crate) address: Address,
    pub(crate) descriptor: Descriptor,
    pub(crate) endpoints: BTreeMap<Endpoint, Option<SimpleDescriptor>>,
}

impl From<super::Device> for Device {
    fn from(device: super::Device) -> Self {
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
