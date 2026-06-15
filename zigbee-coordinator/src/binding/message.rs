use std::collections::BTreeMap;

use zigbee::{Address, Endpoint};
use zigbee_hw::Event;

use crate::discovery::EndpointInfo;
use crate::network_manager::Device;

/// Messages received by the binding management actor.
#[derive(Debug)]
pub enum Message {
    /// A hardware-level event.
    Event(Event),

    /// Information that a certain device has been updated.
    DeviceDiscovered {
        /// The address of the device that has been updated.
        address: Address,
        /// The new device endpoints, keyed by endpoint ID.
        endpoints: BTreeMap<Endpoint, EndpointInfo>,
    },
}

impl From<Event> for Message {
    fn from(event: Event) -> Self {
        Self::Event(event)
    }
}
