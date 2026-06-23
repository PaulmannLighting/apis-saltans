use aps::data::Frame;
use zigbee::Address;

/// Events that can occur on the hardware layer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    /// The network is up and running.
    NetworkUp,

    /// The network is down.
    NetworkDown,

    /// The network has been opened for new joins.
    NetworkOpened,

    /// The network has been closed for new joins.
    NetworkClosed,

    /// A new device has joined the network.
    DeviceJoined(Address),

    /// A device has rejoined the network.
    DeviceRejoined {
        /// The address of the joined device.
        address: Address,
        /// Whether the rejoining was secured.
        secured: bool,
    },

    /// A device has left the network.
    DeviceLeft(Address),

    /// Message received from a device.
    MessageReceived {
        /// The PAN ID of the sender.
        src_address: u16,
        /// The APS frame.
        aps_frame: Frame<Vec<u8>>,
    },
}
