use std::collections::BTreeMap;
use std::fmt::Display;

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

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.address.fmt(f)
    }
}
