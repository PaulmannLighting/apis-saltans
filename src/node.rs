//! Zigbee network node representation.

use alloc::collections::BTreeSet;

use capability::Capability;
use descriptor::Descriptor;
use macaddr::MacAddr8;

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
