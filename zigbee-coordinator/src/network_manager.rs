use std::collections::BTreeMap;

use log::{debug, error};
use macaddr::MacAddr8;
use tokio::sync::mpsc::Receiver;
use zigbee::Address;

pub use self::message::Message;
pub use self::state::{Attributes, Device, Endpoint, State};

mod message;
mod state;

/// The network management actor.
#[derive(Debug, Default)]
pub struct Actor {
    devices: BTreeMap<MacAddr8, Device>,
    short_ids: BTreeMap<u16, MacAddr8>,
}

impl Actor {
    /// Create a new actor.
    #[must_use]
    pub fn new(state: State) -> Self {
        let mut short_ids = BTreeMap::new();
        let devices = state
            .devices
            .into_iter()
            .map(|device| {
                let short_id = device.address.short_id();
                let ieee_address = device.address.ieee_address();
                short_ids.insert(short_id, ieee_address);
                (ieee_address, device)
            })
            .collect();

        Self { devices, short_ids }
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
                                .map(|device| device.address.short_id()),
                        )
                        .unwrap_or_else(|error| {
                            error!("Failed to send response: {error:?}");
                        });
                }
                Message::GetDevices { response } => {
                    response.send(self.devices.clone()).unwrap_or_else(|error| {
                        error!("Failed to send response: {error:?}");
                    });
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
        let ieee_address = device.address.ieee_address();
        let short_id = device.address.short_id();
        self.devices.insert(ieee_address, device);
        self.short_ids.insert(short_id, ieee_address);
    }

    fn remove_device(&mut self, address: &Address) {
        self.devices.remove(&address.ieee_address());
        self.short_ids.remove(&address.short_id());
    }
}
