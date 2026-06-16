use std::collections::BTreeMap;

use log::{debug, error};
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
                    debug!("Received event: {event:?}");
                }
                Message::GetIeeeAddressFromShortId { short_id, response } => {
                    response
                        .send(self.short_ids.get(&short_id).copied())
                        .unwrap_or_else(|error| {
                            error!("Failed to send response: {error:?}");
                        });
                }
                Message::GetShortIdFromIeeeAddress {
                    ieee_address,
                    response,
                } => {
                    response
                        .send(
                            self.devices
                                .get(&ieee_address)
                                .map(|device| device.address().short_id()),
                        )
                        .unwrap_or_else(|error| {
                            error!("Failed to send response: {error:?}");
                        });
                }
                Message::GetDevices { .. } => {
                    todo!()
                }
                Message::Subscribe { .. } => {
                    todo!()
                }
                Message::NewDevice(device) => {
                    self.add_device(device);
                }
                Message::RemoveDevice(address) => {
                    self.remove_device(&address);
                }
            }
        }
    }

    fn add_device(&mut self, device: Device) {
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
