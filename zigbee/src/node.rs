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
    pan_id: u16,
    ieee_address: MacAddr8,
    descriptor: Descriptor,
}

impl Node {
    /// Create a new Zigbee node.
    #[must_use]
    pub const fn new(ieee_address: MacAddr8, pan_id: u16, descriptor: Descriptor) -> Self {
        Self {
            pan_id,
            ieee_address,
            descriptor,
        }
    }

    /// Return the PAN ID of the node.
    #[must_use]
    pub const fn pan_id(&self) -> u16 {
        self.pan_id
    }

    /// Return the IEEE address of the node.
    #[must_use]
    pub const fn ieee_address(&self) -> MacAddr8 {
        self.ieee_address
    }

    /// Return the descriptor of the node.
    #[must_use]
    pub const fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
}
