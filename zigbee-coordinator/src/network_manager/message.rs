use std::collections::BTreeMap;

use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use zigbee::{Address, Endpoint};
use zigbee_hw::Event;

use super::Device;
use crate::discovery::EndpointInfo;

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
    /// A new device has been discovered.
    NewDevice {
        /// The address of the new device.
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
