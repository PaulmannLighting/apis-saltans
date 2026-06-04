use macaddr::MacAddr8;

/// The binding management actor.
pub struct Actor {}

/// Messages received by the binding management actor.
#[expect(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum Message {
    /// A new device has joined the network.
    DeviceJoined {
        /// The IEEE address of the joined device.
        ieee_address: MacAddr8,
        /// The short ID of the joined device.
        short_id: u16,
    },
    /// A device has rejoined the network.
    DeviceRejoined {
        /// The IEEE address of the joined device.
        ieee_address: MacAddr8,
        /// The short ID of the joined device.
        short_id: u16,
        /// Whether the rejoin was secured.
        secured: bool,
    },
    /// A device has left the network.
    DeviceLeft {
        /// The IEEE address of the left device.
        ieee_address: MacAddr8,
        /// The short ID of the left device.
        short_id: u16,
    },
}
