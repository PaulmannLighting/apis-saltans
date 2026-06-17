use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use zigbee::{Address, Endpoint};

use super::Endpoint as EndpointInfo;

/// A Zigbee network device.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Device {
    /// The full address of the device.
    pub address: Address,

    /// The endpoints of the device.
    pub endpoints: BTreeMap<Endpoint, EndpointInfo>,
}

impl From<(Address, BTreeMap<Endpoint, EndpointInfo>)> for Device {
    fn from((address, endpoints): (Address, BTreeMap<Endpoint, EndpointInfo>)) -> Self {
        Self { address, endpoints }
    }
}
