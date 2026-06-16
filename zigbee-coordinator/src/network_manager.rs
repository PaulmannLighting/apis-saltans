use std::collections::BTreeMap;

use macaddr::MacAddr8;
use tokio::sync::mpsc::Receiver;
use zigbee::Address;

pub use self::device::Device;
pub use self::message::Message;

mod device;
mod message;

/// The network management actor.
#[derive(Debug, Default)]
pub struct Actor {
    devices: BTreeMap<MacAddr8, Device>,
    short_ids: BTreeMap<u16, MacAddr8>,
}

impl Actor {
    /// Create a new actor.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            devices: BTreeMap::new(),
            short_ids: BTreeMap::new(),
        }
    }

    /// Run the actor.
    pub async fn run(mut self, mut messages: Receiver<Message>) {
        while let Some(message) = messages.recv().await {
            match message {
                Message::Event(event) => {
                    todo!()
                }
                Message::GetDevices { .. } => {
                    todo!()
                }
                Message::Subscribe { .. } => {
                    todo!()
                }
                Message::NewDevice(device) => {
                    self.add_new_device(device);
                }
                Message::RemoveDevice(address) => {
                    self.remove_device(&address);
                }
            }
        }
    }

    fn add_new_device(&mut self, device: Device) {
        let address = device.address();
        let ieee_address = address.ieee_address();
        let short_id = address.short_id();
        self.devices.insert(ieee_address, device);
        self.short_ids.insert(short_id, ieee_address);
    }

    fn remove_device(&mut self, address: &Address) {
        self.devices.remove(&address.ieee_address());
        self.short_ids.remove(&address.short_id());
    }
}
