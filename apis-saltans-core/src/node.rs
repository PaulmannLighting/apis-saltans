//! Zigbee network node representation.

pub use self::descriptor::{
    Descriptor, DeviceType, Flags, FrequencyBand, LogicalType, MacCapabilityFlags, ServerMask,
};
use crate::IeeeAddress;

mod descriptor;

/// A Zigbee node.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct Node {
    short_id: u16,
    ieee_address: IeeeAddress,
    descriptor: Descriptor,
}

impl Node {
    /// Create a new Zigbee node.
    #[must_use]
    pub const fn new(ieee_address: IeeeAddress, short_id: u16, descriptor: Descriptor) -> Self {
        Self {
            short_id,
            ieee_address,
            descriptor,
        }
    }

    /// Return the short ID of the node.
    #[must_use]
    pub const fn short_id(&self) -> u16 {
        self.short_id
    }

    /// Return the IEEE address of the node.
    #[must_use]
    pub const fn ieee_address(&self) -> IeeeAddress {
        self.ieee_address
    }

    /// Return the descriptor of the node.
    #[must_use]
    pub const fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
}
