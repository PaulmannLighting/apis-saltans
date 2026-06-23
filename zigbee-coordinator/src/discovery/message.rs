use zigbee::Address;
use zigbee::node::MacCapabilityFlags;

/// Message sent to the discovery actor.
#[derive(Debug)]
#[expect(clippy::enum_variant_names)]
pub enum Message {
    /// A device has joined the network.
    DeviceJoined(Address),

    /// A device has rejoined the network.
    DeviceRejoined {
        /// The address of the device.
        address: Address,
        /// Whether the join was secured.
        secured: bool,
    },

    /// A device has been announced.
    DeviceAnnounced {
        /// The address of the device.
        address: Address,
        /// The capabilities of the device.
        capabilities: MacCapabilityFlags,
    },
}
