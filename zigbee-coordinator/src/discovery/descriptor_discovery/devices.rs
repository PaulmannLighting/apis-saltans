use std::collections::BTreeMap;

use zdp::SimpleDescriptor;
use zigbee::{Address, Endpoint};

/// Type alias for a map of devices to their endpoints.
pub type Devices = BTreeMap<Address, BTreeMap<Endpoint, Option<SimpleDescriptor>>>;
