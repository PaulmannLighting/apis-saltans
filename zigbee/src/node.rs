//! Zigbee network node representation.

use std::collections::BTreeSet;

use macaddr::MacAddr8;

pub use self::capability::Capability;
pub use self::descriptor::{Descriptor, Flags, FrequencyBand, MacCapabilityFlags, ServerMask};

mod capability;
mod descriptor;

/// A Zigbee node.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct Node {
    address: MacAddr8,
    short_address: Option<u16>,
    capabilities: BTreeSet<Capability>,
    descriptor: Descriptor,
}

impl Node {
    /// Create a new Zigbee node.
    #[must_use]
    pub const fn new(
        address: MacAddr8,
        short_address: Option<u16>,
        capabilities: BTreeSet<Capability>,
        descriptor: Descriptor,
    ) -> Self {
        Self {
            address,
            short_address,
            capabilities,
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

    /// Return the capabilities of the node.
    #[must_use]
    pub const fn capabilities(&self) -> &BTreeSet<Capability> {
        &self.capabilities
    }

    /// Return the descriptor of the node.
    #[must_use]
    pub const fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
}
