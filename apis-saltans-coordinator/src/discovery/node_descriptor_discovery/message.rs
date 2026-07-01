use apis_saltans_core::Address;
use apis_saltans_core::node::Descriptor;

/// Message sent to the descriptor discovery actor.
#[derive(Debug)]
pub enum Message {
    /// Discover descriptors on the given device.
    Discover(Address),

    /// Descriptor for the given device has been discovered.
    DescriptorDiscovered {
        /// The device that the descriptor belongs to.
        address: Address,
        /// The discovered descriptor.
        descriptor: Box<Descriptor>,
    },

    /// Discovery of the descriptor for the given device failed.
    DiscoveryFailed(Address),
}
