use std::collections::{BTreeMap, BTreeSet};

use zigbee::{Address, Endpoint};

/// Type alias for the device map.
pub type Endpoints = BTreeMap<Address, BTreeSet<Endpoint>>;
