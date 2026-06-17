use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use zigbee::{Address, Endpoint};

use crate::Endpoint as EndpointInfo;

/// A Zigbee network device.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Device {
    pub address: Address,
    pub endpoints: BTreeMap<Endpoint, EndpointInfo>,
}

impl From<(Address, BTreeMap<Endpoint, EndpointInfo>)> for Device {
    fn from((address, endpoints): (Address, BTreeMap<Endpoint, EndpointInfo>)) -> Self {
        Self { address, endpoints }
    }
}
