use std::collections::BTreeSet;
use std::fmt::Display;

use apis_saltans_core::node::Descriptor;
use apis_saltans_core::{Endpoint, FullAddress};

#[derive(Debug)]
pub struct IncomingDevice {
    pub(crate) address: FullAddress,
    pub(crate) descriptor: Descriptor,
    pub(crate) endpoints: BTreeSet<Endpoint>,
}

impl IncomingDevice {
    /// Create a new instance of `Device`.
    #[must_use]
    pub const fn new(
        address: FullAddress,
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

impl Display for IncomingDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.address.fmt(f)
    }
}
