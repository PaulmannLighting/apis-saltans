use std::collections::BTreeSet;

use zigbee::{Address, Endpoint};

/// Message sent to the endpoint discovery actor.
#[cfg_attr(target_pointer_width = "64", expect(variant_size_differences))]
#[derive(Debug)]
pub enum Message {
    /// Discover endpoints on the given device.
    Discover(Address),

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
