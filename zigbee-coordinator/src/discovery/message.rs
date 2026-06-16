use zigbee::Address;

/// Message sent to the discovery actor.
#[derive(Debug)]
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
}
