use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, BTreeSet};

use apis_saltans_aps::Data;
use apis_saltans_core::Address;
use apis_saltans_hw::Ncp;
use apis_saltans_zcl::{Cluster, Frame};
use log::{debug, error, info, warn};
use macaddr::MacAddr8;
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender, channel};

pub use self::message::Message;
pub use self::state::{Attributes, Device, Endpoint, State};
use crate::{Event, MPSC_CHANNEL_SIZE};

mod message;
mod state;

/// The network management actor.
#[derive(Debug)]
pub struct Actor<T> {
    ncp: T,
    devices: BTreeMap<MacAddr8, Device>,
    short_ids: BTreeMap<u16, MacAddr8>,
    subscribers: Vec<(BTreeSet<MacAddr8>, Sender<Event>)>,
}

impl<T> Actor<T>
where
    T: Ncp + Send + Sync,
{
    /// Create a new actor.
    #[must_use]
    pub const fn new(ncp: T) -> Self {
        Self {
            ncp,
            devices: BTreeMap::new(),
            short_ids: BTreeMap::new(),
            subscribers: Vec::new(),
        }
    }

    /// Run the actor.
    pub async fn run(mut self, mut messages: Receiver<Message>) {
        while let Some(message) = messages.recv().await {
            match message {
                Message::Load(state) => {
                    self.load(state);
                }
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
                Message::DeviceJoined { address, secured } => {
                    if let Some(secured) = secured {
                        debug!("Device rejoined the network: {address}, secured: {secured}");
                    } else {
                        debug!("Device joined the network: {address}");
                    }
                }
                Message::NewDevice(device) => {
                    info!("New device: {device:?}");
                    self.add_device(device).await;
                }
                Message::RemoveDevice(address) => {
                    self.remove_device(&address);
                }
                Message::RouteError(route_error) => {
                    warn!("{route_error}");
                    self.ncp.route_request(64).await.unwrap_or_else(|error| {
                        error!("Failed to request route: {error:?}");
                    });
                }
                Message::NetworkOpened => {
                    info!("Network opened");
                }
                Message::NetworkClosed => {
                    info!("Network closed");
                }
            }
        }
    }

    fn load(&mut self, state: Box<[Device]>) {
        for device in state {
            self.short_ids
                .insert(device.address.short_id(), device.address.ieee_address());

            match self.devices.entry(device.address.ieee_address()) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().endpoints = device.endpoints;
                }
                Entry::Vacant(entry) => {
                    entry.insert(device);
                }
            }
        }
    }

    async fn handle_incoming_command(&mut self, src_address: u16, frame: Data<Frame<Cluster>>) {
        let Some(ieee_address) = self.short_ids.get(&src_address) else {
            warn!("Received command from unknown short ID: {src_address:04X}");
            return;
        };

        let (aps_header, zcl_frame) = frame.into_parts();
        let (_zcl_header, cluster) = zcl_frame.into_parts();

        for subscriber in self.subscribers.iter().filter_map(|(devices, sender)| {
            if devices.is_empty() || devices.contains(ieee_address) {
                Some(sender)
            } else {
                None
            }
        }) {
            subscriber
                .send(Event::new(
                    Address::new(*ieee_address, src_address),
                    aps_header.source_endpoint(),
                    cluster.clone(),
                ))
                .await
                .unwrap_or_else(|error| {
                    debug!("Failed to send command to subscriber: {error:?}");
                });
        }

        self.subscribers.retain(|(_, sender)| !sender.is_closed());
    }

    async fn add_device(&mut self, device: Device) {
        let ieee_address = device.address.ieee_address();
        let short_id = device.address.short_id();
        self.devices.insert(ieee_address, device);
        self.short_ids.insert(short_id, ieee_address);
        self.ncp.route_request(64).await.unwrap_or_else(|error| {
            error!("Failed to request route: {error:?}");
        });
    }

    fn remove_device(&mut self, address: &Address) {
        self.devices.remove(&address.ieee_address());
        self.short_ids.remove(&address.short_id());
    }
}

impl<T> Actor<T>
where
    T: Ncp + Send + Sync + 'static,
{
    /// Start the network manager.
    pub fn spawn(ncp: T) -> Sender<Message>
    where
        T: Send + Sync + 'static,
    {
        let (tx, rx) = channel(MPSC_CHANNEL_SIZE);
        spawn(Self::new(ncp).run(rx));
        tx
    }
}
