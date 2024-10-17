use macaddr::MacAddr8;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use capability::Capability;
use descriptor::Descriptor;

mod capability;
mod descriptor;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Node {
    address: MacAddr8,
    short_address: Option<u16>,
    capabilities: HashSet<Capability>,
    descriptor: Descriptor,
}
