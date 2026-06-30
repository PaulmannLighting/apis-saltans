use std::collections::BTreeMap;

use zdp::SimpleDescriptor;
use zigbee::node::Descriptor;
use zigbee::{Address, Endpoint};

#[derive(Debug)]
pub struct Device {
    pub address: Address,
    pub descriptor: Descriptor,
    pub endpoints: BTreeMap<Endpoint, SimpleDescriptor>,
}

impl Device {
    #[must_use]
    pub const fn new(
        address: Address,
        descriptor: Descriptor,
        endpoints: BTreeMap<Endpoint, SimpleDescriptor>,
    ) -> Self {
        Self {
            address,
            descriptor,
            endpoints,
        }
    }
}
