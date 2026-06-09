use std::collections::BTreeMap;

use zigbee::Address;

pub use self::endpoint::Endpoint;

mod endpoint;

/// A Zigbee network device.
#[derive(Debug)]
pub struct Device {
    address: Address,
    endpoints: BTreeMap<zigbee::Endpoint, Endpoint>,
}
