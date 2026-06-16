use macaddr::MacAddr8;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use zigbee::Address;
use zigbee_hw::Event;

use super::Device;

/// Messages received by the network management actor.
#[derive(Debug)]
pub enum Message {
    /// A hardware-level event.
    Event(Event),

    /// A request to resolve a short ID to an IEEE address.
    GetIeeeAddressFromShortId {
        /// The short ID to resolve.
        short_id: u16,
        /// Response channel to send the resolved IEEE address to.
        response: oneshot::Sender<Option<MacAddr8>>,
    },

    /// A request to resolve an IEEE address to a short ID.
    GetShortIdFromIeeeAddress {
        /// The IEEE address to resolve.
        ieee_address: MacAddr8,
        /// Response channel to send the resolved IEEE address to.
        response: oneshot::Sender<Option<u16>>,
    },

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

    /// Add a new device to the network.
    NewDevice(Device),

    /// Remove a device from the network.
    RemoveDevice(Address),
}

impl From<Event> for Message {
    fn from(event: Event) -> Self {
        Self::Event(event)
    }
}
