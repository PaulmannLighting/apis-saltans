use aps::Data;
use macaddr::MacAddr8;

pub use self::command::Command;
use crate::{FoundNetwork, ScannedChannel};

mod command;

/// Events that can occur on the hardware layer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    /// Channel found.
    ChannelFound {
        /// The channel that has been found.
        channels: Vec<ScannedChannel>,
    },
    /// Network found.
    NetworkFound {
        /// The network that has been found.
        networks: Vec<FoundNetwork>,
    },
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
    /// A new device has been discovered.
    DeviceDiscovered {
        /// The IEEE address of the discovered device.
        ieee_address: MacAddr8,
        /// The short ID of the discovered device.
        short_id: u16,
    },
    /// Message received from a device.
    MessageReceived {
        /// The PAN ID of the sender.
        src_address: u16,
        /// The APS frame.
        aps_frame: Box<Data<Command>>,
    },
}
