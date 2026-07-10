use std::collections::BTreeMap;
use std::fmt::Display;

use apis_saltans_core::node::Descriptor;
use apis_saltans_core::{Endpoint, FullAddress};
use apis_saltans_zdp::SimpleDescriptor;

#[derive(Debug)]
pub struct IncomingDevice {
    pub address: FullAddress,
    pub descriptor: Descriptor,
    pub endpoints: BTreeMap<Endpoint, SimpleDescriptor>,
}

impl IncomingDevice {
    #[must_use]
    pub const fn new(
        address: FullAddress,
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

impl Display for IncomingDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.address.fmt(f)
    }
}
