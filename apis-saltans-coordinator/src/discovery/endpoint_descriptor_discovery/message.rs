use apis_saltans_core::FullAddress;
use apis_saltans_zdp::SimpleDescriptor;

use super::IncomingDevice;

/// Message sent to the descriptor discovery actor.
#[derive(Debug)]
pub enum Message {
    /// Discover descriptors for the given endpoints.
    Discover(IncomingDevice),

    /// Get the descriptor for the given endpoint.
    DescriptorDiscovered {
        /// The device to get the descriptor for.
        address: FullAddress,
        /// The number of retries.
        descriptor: Box<SimpleDescriptor>,
    },

    /// Discovery of the given device has failed.
    DiscoveryFailed(FullAddress),
}
