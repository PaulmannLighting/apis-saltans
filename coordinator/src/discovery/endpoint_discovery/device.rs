use std::fmt::Display;

use zb_core::FullAddress;
use zb_core::node::Descriptor;

#[derive(Debug)]
pub struct Device {
    pub(crate) address: FullAddress,
    pub(crate) descriptor: Descriptor,
}

impl Device {
    #[must_use]
    pub const fn new(address: FullAddress, descriptor: Descriptor) -> Self {
        Self {
            address,
            descriptor,
        }
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.address.fmt(f)
    }
}
