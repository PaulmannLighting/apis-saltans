use log::{debug, error, info, trace, warn};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use zb_aps::Data;
use zb_core::{FullAddress, IeeeAddress};
use zb_hw::Ncp;
use zb_nwk::Source;
use zb_zcl::{Cluster, Frame};

pub use self::message::Message;
use crate::event::Zcl;
use crate::network::Device;
use crate::storage::Storage;
use crate::{Error, Event, MPSC_CHANNEL_SIZE, storage};

mod message;

/// The network management actor.
#[derive(Debug)]
pub struct Actor<T> {
    ncp: T,
    storage: Sender<storage::Message>,
    subscribers: Vec<Sender<Event>>,
}

impl<T> Actor<T>
where
    T: Ncp + Send + Sync,
{
    /// Create a new actor.
    #[must_use]
    pub const fn new(ncp: T, storage: Sender<storage::Message>) -> Self {
        Self {
            ncp,
            storage,
            subscribers: Vec::new(),
        }
    }

    /// Run the actor.
    pub async fn run(mut self, mut messages: Receiver<Message>) {
        // TODO: Inform sub-actors about currently not fully discovered devices.
        // Pass in desired state to sub-actor where applicable.

        while let Some(message) = messages.recv().await {
            match message {
                Message::Subscribe(sender) => {
                    self.subscribers.push(sender);
                }
                Message::Command { source, frame } => {
                    self.handle_incoming_command(source, frame).await;
                }
                Message::GetIeeeAddressFromShortId { short_id, response } => {
                    response
                        .send(self.storage.get_ieee_address(short_id).await.ok().flatten())
                        .unwrap_or_else(|error| {
                            error!("Failed to send response: {error:?}");
                        });
                }
                Message::GetShortIdFromIeeeAddress {
                    ieee_address,
                    response,
                } => {
                    response
                        .send(self.storage.get_short_id(ieee_address).await.ok().flatten())
                        .unwrap_or_else(|error| {
                            error!("Failed to send response: {error:?}");
                        });
                }
                Message::DeviceJoined { address, secured } => {
                    if let Some(secured) = secured {
                        debug!("Device rejoined the network: {address}, secured: {secured}");
                    } else {
                        debug!("Device joined the network: {address}");
                    }

                    self.storage
                        .update_short_id(address.ieee_address(), address.short_id())
                        .await
                        .unwrap_or_else(|error| {
                            error!("Failed to update the network: {error:?}");
                        });
                }
                Message::NewDevice { address, device } => {
                    info!("New device {address}: {device:?}");
                    self.add_device(address, device).await;
                }
                Message::RemoveDevice(address) => {
                    self.remove_device(address).await;
                }
                Message::RouteError(route_error) => {
                    warn!("{route_error}");
                    self.ncp.route_request(64).await.unwrap_or_else(|error| {
                        error!("Failed to request route: {error:?}");
                    });
                }
                Message::GetDevices(sender) => {
                    sender
                        .send(self.devices().await.unwrap_or_default())
                        .unwrap_or_else(drop);
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

    async fn devices(&self) -> Result<Box<[Device]>, Error> {
        Ok(self.storage.devices().await?)
    }

    async fn handle_incoming_command(&mut self, source: Source, frame: Data<Frame<Cluster>>) {
        let Some(address) = self.get_address_from_source(source).await else {
            warn!("Received command from unknown short ID: {source}");
            return;
        };

        self.mux_event(Event::Zcl(Zcl::new(address, frame))).await;
    }

    async fn get_address_from_source(&self, source: Source) -> Option<FullAddress> {
        let Ok(short_id) = source.node_id().try_into().inspect_err(|error| {
            warn!("Received invalid node ID: {error:?}");
        }) else {
            return None;
        };

        if let Some(ieee_address) = source.ieee_address() {
            return Some(FullAddress::new(ieee_address, short_id));
        }

        let Ok(short_id) = source.node_id().try_into().inspect_err(|error| {
            warn!("Received invalid node ID: {error:?}");
        }) else {
            return None;
        };

        trace!("NWK source does not supply source IEEE address. Querying storage.");
        let Some(ieee_address) = self
            .storage
            .get_ieee_address(short_id)
            .await
            .inspect_err(|error| error!("{error}"))
            .ok()
            .flatten()
        else {
            warn!("Device {short_id} is not known to storage.");
            return None;
        };

        Some(FullAddress::new(ieee_address, short_id))
    }

    async fn add_device(&self, address: FullAddress, device: Device) {
        self.storage.add(address, device).await.map_or_else(
            |error| {
                error!("Failed to store device: {error:?}");
            },
            |device| {
                if let Some(device) = device {
                    debug!("Replaced existing device: {device:?}");
                }
            },
        );

        self.ncp.route_request(64).await.unwrap_or_else(|error| {
            error!("Failed to request route: {error:?}");
        });
    }

    async fn remove_device(&self, ieee_address: IeeeAddress) {
        self.storage.remove(ieee_address).await.map_or_else(
            |error| {
                error!("Failed to remove device: {error:?}");
            },
            |device| {
                if let Some(device) = device {
                    debug!("Removed device: {device:?}");
                }
            },
        );
    }

    async fn mux_event(&mut self, event: Event) {
        for subscriber in &self.subscribers {
            subscriber
                .send(event.clone())
                .await
                .unwrap_or_else(|error| {
                    debug!("Failed to send command to subscriber: {error:?}");
                });
        }

        self.subscribers.retain(|sender| !sender.is_closed());
    }

    /// Start the network manager.
    pub fn spawn(ncp: T, storage: Sender<storage::Message>) -> Sender<Message>
    where
        Self: 'static,
    {
        let (tx, rx) = channel(MPSC_CHANNEL_SIZE);
        spawn(Self::new(ncp, storage).run(rx));
        tx
    }
}
