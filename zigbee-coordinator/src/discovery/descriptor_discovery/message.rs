use std::collections::BTreeSet;

use zdp::SimpleDescriptor;
use zigbee::{Address, Endpoint};

/// Message sent to the descriptor discovery actor.
#[derive(Debug)]
pub enum Message {
    /// Discover descriptors for the given endpoints.
    Discover {
        /// The device to discover descriptors for.
        address: Address,
        /// The endpoints to discover descriptors for.
        endpoints: BTreeSet<Endpoint>,
    },

    /// Get the descriptor for the given endpoint.
    DescriptorsDiscovered {
        /// The device to get the descriptor for.
        address: Address,
        /// The number of retries.
        descriptors: Box<[SimpleDescriptor]>,
    },
}
