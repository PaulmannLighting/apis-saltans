use zigbee_hw::Event;

use crate::network_manager::Device;

/// Messages received by the binding management actor.
#[derive(Debug)]
pub enum Message {
    /// A hardware-level event.
    Event(Event),
    /// Information about a device.
    Device(Device),
}

impl From<Event> for Message {
    fn from(event: Event) -> Self {
        Self::Event(event)
    }
}

impl From<Device> for Message {
    fn from(device: Device) -> Self {
        Self::Device(device)
    }
}
