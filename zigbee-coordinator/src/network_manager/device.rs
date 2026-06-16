use std::collections::BTreeMap;

use zigbee::{Address, Endpoint};

use crate::discovery::EndpointInfo;

/// A Zigbee network device.
#[derive(Debug)]
pub struct Device {
    address: Address,
    endpoints: BTreeMap<Endpoint, EndpointInfo>,
}

impl Device {
    /// Return the device's address.
    #[must_use]
    pub const fn address(&self) -> &Address {
        &self.address
    }

    /// Return the device's endpoints.
    #[must_use]
    pub const fn endpoints(&self) -> &BTreeMap<Endpoint, EndpointInfo> {
        &self.endpoints
    }
}

impl From<(Address, BTreeMap<Endpoint, EndpointInfo>)> for Device {
    fn from((address, endpoints): (Address, BTreeMap<Endpoint, EndpointInfo>)) -> Self {
        Self { address, endpoints }
    }
}
