//! Zigbee network node representation.

use std::collections::BTreeSet;

use macaddr::MacAddr8;

pub use self::capability::Capability;
pub use self::descriptor::{Descriptor, Flags, MacCapabilityFlags};

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
