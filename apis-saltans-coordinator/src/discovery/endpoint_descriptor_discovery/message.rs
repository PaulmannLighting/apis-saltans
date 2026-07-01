use apis_saltans_zdp::SimpleDescriptor;
use apis_saltans_core::Address;

use super::Device;

/// Message sent to the descriptor discovery actor.
#[derive(Debug)]
pub enum Message {
    /// Discover descriptors for the given endpoints.
    Discover(Device),

    /// Get the descriptor for the given endpoint.
    DescriptorDiscovered {
        /// The device to get the descriptor for.
        address: Address,
        /// The number of retries.
        descriptor: Box<SimpleDescriptor>,
    },

    /// Discovery of the given device has failed.
    DiscoveryFailed(Address),
}
