//! Zigbee network node representation.

use macaddr::MacAddr8;

pub use self::descriptor::{
    Descriptor, DeviceType, Flags, FrequencyBand, LogicalType, MacCapabilityFlags, ServerMask,
};

mod descriptor;

/// A Zigbee node.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct Node {
    address: MacAddr8,
    short_address: Option<u16>,
    descriptor: Descriptor,
}

impl Node {
    /// Create a new Zigbee node.
    #[must_use]
    pub const fn new(
        address: MacAddr8,
        short_address: Option<u16>,
        descriptor: Descriptor,
    ) -> Self {
        Self {
            address,
            short_address,
            descriptor,
        }
    }

    /// Return the MAC address of the node.
    #[must_use]
    pub const fn address(&self) -> MacAddr8 {
        self.address
    }

    /// Return the short address of the node, if available.
    #[must_use]
    pub const fn short_address(&self) -> Option<u16> {
        self.short_address
    }

    /// Return the descriptor of the node.
    #[must_use]
    pub const fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
}
