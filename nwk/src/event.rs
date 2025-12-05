pub use aps_frame::ReceivedApsFrame;
use macaddr::MacAddr8;

mod aps_frame;

/// Events that can occur in the network module.
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
    DeviceJoined {
        /// The IEEE address of the joined device.
        ieee_address: MacAddr8,
        /// The PAN ID of the joined device.
        pan_id: u16,
    },
    /// A device has left the network.
    DeviceLeft {
        /// The IEEE address of the left device.
        ieee_address: MacAddr8,
        /// The PAN ID of the left device.
        pan_id: u16,
    },
    /// A new device has been discovered.
    DeviceDiscovered {
        /// The IEEE address of the discovered device.
        ieee_address: MacAddr8,
        /// The PAN ID of the discovered device.
        pan_id: u16,
    },
    /// Message received from a device.
    MessageReceived(ReceivedApsFrame),
}
