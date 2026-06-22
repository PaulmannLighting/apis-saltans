use std::collections::{BTreeMap, BTreeSet};

use log::{debug, error, warn};
use macaddr::MacAddr8;
use tokio::sync::mpsc::{Receiver, Sender};
use zcl::Cluster;
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
    subscribers: Vec<(BTreeSet<MacAddr8>, Sender<Cluster>)>,
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

        Self {
            devices,
            short_ids,
            subscribers: Vec::new(),
        }
    }

    /// Run the actor.
    pub async fn run(mut self, mut messages: Receiver<Message>) {
        while let Some(message) = messages.recv().await {
            match message {
                Message::SubscribeToIncomingCommands { devices, sender } => {
                    self.subscribers.push((devices, sender));
                }
                Message::Command {
                    src_address,
                    payload,
                } => {
                    self.handle_incoming_command(src_address, *payload).await;
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
                Message::SubscribeToDevice { .. } => {
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

    async fn handle_incoming_command(&mut self, src_address: u16, payload: Cluster) {
        let Some(src_address) = self.short_ids.get(&src_address) else {
            warn!("Received command from unknown short ID: {src_address:04X}");
            return;
        };

        for subscriber in self.subscribers.iter().filter_map(|(devices, sender)| {
            if devices.is_empty() || devices.contains(src_address) {
                Some(sender)
            } else {
                None
            }
        }) {
            subscriber
                .send(payload.clone().into())
                .await
                .unwrap_or_else(|error| {
                    debug!("Failed to send command to subscriber: {error:?}");
                });
        }

        self.subscribers.retain(|(_, sender)| !sender.is_closed());
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
