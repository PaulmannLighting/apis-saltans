use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use zigbee_hw::Event;

use crate::network_manager::Device;

/// Messages received by the network management actor.
#[derive(Debug)]
pub enum Message {
    /// A hardware-level event.
    Event(Event),
    /// A request to send a list of the current devices.
    GetDevices {
        /// Response channel to send the current device list to.
        response: oneshot::Sender<Box<[Device]>>,
    },
    /// A request to subscribe for updates on devices.
    Subscribe {
        /// Response channel to send the updated device list to.
        response: Sender<Box<[Device]>>,
    },
    /// Information that a certain device has been updated.
    DeviceUpdate {
        /// The updated device.
        device: Device,
    },
}

impl From<Event> for Message {
    fn from(event: Event) -> Self {
        Self::Event(event)
    }
}
