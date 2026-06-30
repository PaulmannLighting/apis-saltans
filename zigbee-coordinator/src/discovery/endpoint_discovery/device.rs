use zigbee::Address;
use zigbee::node::Descriptor;

#[derive(Debug)]
pub struct Device {
    pub(crate) address: Address,
    pub(crate) descriptor: Descriptor,
}

impl Device {
    #[must_use]
    pub const fn new(address: Address, descriptor: Descriptor) -> Self {
        Self {
            address,
            descriptor,
        }
    }
}
