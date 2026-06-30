use std::collections::{BTreeMap, BTreeSet};

use log::{debug, error, trace, warn};
use tokio::sync::mpsc::{Receiver, Sender, WeakSender, channel};
use tokio_task_pool::Pool;
use zdp::SimpleDescriptor;
use zigbee::{Address, Endpoint};

use self::devices::Devices;
pub use self::message::Message;
use crate::discovery::attribute_discovery;
use crate::discovery::descriptor_discovery::discovery_task::DiscoveryTask;
use crate::{MPSC_CHANNEL_SIZE, TASK_POOL_SIZE, transceiver};

mod devices;
mod discovery_task;
mod message;

/// Actor to discover descriptors on devices.
#[derive(Debug)]
pub struct DescriptorDiscovery {
    inbox: Receiver<Message>,
    loopback: WeakSender<Message>,
    zdp: WeakSender<transceiver::zdp::Message>,
    attribute_discovery: Sender<attribute_discovery::Message>,
    devices: Devices,
    tasks: Pool,
}

impl DescriptorDiscovery {
    /// Create a new instance of `DescriptorDiscovery`.
    #[must_use]
    pub fn new(
        zdp: WeakSender<transceiver::zdp::Message>,
        attribute_discovery: Sender<attribute_discovery::Message>,
    ) -> (Self, Sender<Message>) {
        let (tx, rx) = channel(MPSC_CHANNEL_SIZE);
        let instance = Self {
            inbox: rx,
            loopback: tx.downgrade(),
            zdp,
            attribute_discovery,
            devices: BTreeMap::new(),
            tasks: Pool::bounded(TASK_POOL_SIZE),
        };
        (instance, tx)
    }

    /// Run the actor.
    pub async fn run(mut self) {
        while let Some(message) = self.inbox.recv().await {
            match message {
                Message::Discover { address, endpoints } => {
                    self.discover(&address, endpoints).await;
                }
                Message::DescriptorDiscovered {
                    address,
                    descriptor,
                } => {
                    self.descriptor_discovered(address, *descriptor).await;
                }
                Message::DiscoveryFailed(address) => {
                    if self.devices.remove(&address).is_some() {
                        trace!("Removed failed discovery of: {address}");
                    }
                }
            }
        }
    }

    /// Discover the descriptors for the given endpoints.
    async fn discover(&mut self, address: &Address, endpoints: BTreeSet<Endpoint>) {
        if self.devices.contains_key(address) {
            trace!("Already discovering descriptors for {address}");
            return;
        }

        self.devices.insert(
            address.clone(),
            endpoints.iter().map(|endpoint| (*endpoint, None)).collect(),
        );

        let Some(loopback) = self.loopback.upgrade() else {
            warn!("Failed to upgrade loopback channel.");
            return;
        };

        let Some(zdp) = self.zdp.upgrade() else {
            warn!("Failed to upgrade ZDP channel.");
            return;
        };

        for endpoint in endpoints {
            self.tasks
                .spawn(
                    DiscoveryTask::new(address.clone(), endpoint, loopback.clone(), zdp.clone())
                        .run(),
                )
                .await
                .map_or_else(
                    |error| {
                        error!("Failed to spawn task: {error:?}");
                    },
                    drop,
                );
        }
    }

    /// Update the descriptor map with the newly discovered descriptors.
    async fn descriptor_discovered(&mut self, address: Address, descriptor: SimpleDescriptor) {
        let device = self.devices.entry(address.clone()).or_default();
        device.insert(descriptor.endpoint(), Some(descriptor));
        self.forward_descriptors_if_complete(address).await;
    }

    /// Forward the descriptors if all have been discovered.
    async fn forward_descriptors_if_complete(&mut self, address: Address) {
        let Some(descriptors) = self.devices.get(&address) else {
            error!("Got descriptors for {address} before we discovered them.");
            return;
        };

        if descriptors.values().any(Option::is_none) {
            trace!("Not all descriptors for {address} discovered.");
            return;
        }

        trace!("All descriptors for {address} discovered.");

        let endpoints = self
            .devices
            .remove(&address)
            .expect("We just ensured that this exists above.")
            .into_iter()
            .map(|(endpoint, descriptor)| {
                (
                    endpoint,
                    descriptor.expect("We just ensured that no descriptors are None above."),
                )
            })
            .collect();

        debug!("Forwarding descriptors for {address} to attribute discovery: {endpoints:?}");
        self.attribute_discovery
            .send(attribute_discovery::Message::GetAttributes { address, endpoints })
            .await
            .unwrap_or_else(|error| error!("Failed to send GetAttributes message: {error:?}"));
    }
}
