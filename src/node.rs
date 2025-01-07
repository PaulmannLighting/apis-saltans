use macaddr::MacAddr8;
use std::collections::HashSet;

use capability::Capability;
use descriptor::Descriptor;

mod capability;
mod descriptor;

/// A Zigbee node.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct Node {
    address: MacAddr8,
    short_address: Option<u16>,
    capabilities: HashSet<Capability>,
    descriptor: Descriptor,
}
