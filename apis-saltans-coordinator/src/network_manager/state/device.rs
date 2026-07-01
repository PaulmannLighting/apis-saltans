use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use apis_saltans_core::node::Descriptor;
use apis_saltans_core::{Address, Endpoint};

use super::Endpoint as EndpointInfo;

/// A Zigbee network device.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Device {
    /// The full address of the device.
    pub address: Address,

    /// The device descriptor.
    pub descriptor: Descriptor,

    /// The endpoints of the device.
    pub endpoints: BTreeMap<Endpoint, EndpointInfo>,
}

impl Device {
    /// Create a new device.
    #[must_use]
    pub const fn new(
        address: Address,
        descriptor: Descriptor,
        endpoints: BTreeMap<Endpoint, EndpointInfo>,
    ) -> Self {
        Self {
            address,
            descriptor,
            endpoints,
        }
    }
}
