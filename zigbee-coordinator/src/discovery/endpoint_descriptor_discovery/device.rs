use std::collections::BTreeSet;

use zigbee::node::Descriptor;
use zigbee::{Address, Endpoint};

#[derive(Debug)]
pub struct Device {
    pub(crate) address: Address,
    pub(crate) descriptor: Descriptor,
    pub(crate) endpoints: BTreeSet<Endpoint>,
}

impl Device {
    /// Create a new instance of `Device`.
    #[must_use]
    pub const fn new(
        address: Address,
        descriptor: Descriptor,
        endpoints: BTreeSet<Endpoint>,
    ) -> Self {
        Self {
            address,
            descriptor,
            endpoints,
        }
    }
}
