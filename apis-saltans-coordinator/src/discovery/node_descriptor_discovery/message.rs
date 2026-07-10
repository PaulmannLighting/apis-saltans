use apis_saltans_core::FullAddress;
use apis_saltans_core::node::Descriptor;

/// Message sent to the descriptor discovery actor.
#[derive(Debug)]
pub enum Message {
    /// Discover descriptors on the given device.
    Discover(FullAddress),

    /// Descriptor for the given device has been discovered.
    DescriptorDiscovered {
        /// The device that the descriptor belongs to.
        address: FullAddress,
        /// The discovered descriptor.
        descriptor: Box<Descriptor>,
    },

    /// Discovery of the descriptor for the given device failed.
    DiscoveryFailed(FullAddress),
}
