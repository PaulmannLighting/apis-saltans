use std::collections::BTreeMap;

use apis_saltans_core::Endpoint;
use apis_saltans_core::node::Descriptor;
use serde::{Deserialize, Serialize};

use super::EndpointInfo;

/// A Zigbee network device.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Device {
    /// The device descriptor.
    pub descriptor: Descriptor,

    /// The endpoints of the device.
    pub endpoints: BTreeMap<Endpoint, EndpointInfo>,
}

impl Device {
    /// Create a new device.
    #[must_use]
    pub const fn new(descriptor: Descriptor, endpoints: BTreeMap<Endpoint, EndpointInfo>) -> Self {
        Self {
            descriptor,
            endpoints,
        }
    }
}
