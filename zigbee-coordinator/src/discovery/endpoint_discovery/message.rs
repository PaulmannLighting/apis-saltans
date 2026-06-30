use std::collections::BTreeSet;

use zigbee::{Address, Endpoint};

use super::Device;

/// Message sent to the endpoint discovery actor.
#[derive(Debug)]
pub enum Message {
    /// Discover endpoints on the given device.
    Discover(Device),

    /// Endpoints have been discovered.
    Discovered {
        /// The device that has been discovered.
        address: Address,
        /// The endpoints that have been discovered.
        endpoints: BTreeSet<Endpoint>,
    },

    /// Discovery of the given device has failed.
    DiscoveryFailed(Address),
}
