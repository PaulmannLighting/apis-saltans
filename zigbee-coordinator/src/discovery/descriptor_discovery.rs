use std::collections::{BTreeMap, BTreeSet};

use log::{debug, error, trace, warn};
use tokio::sync::mpsc::{Receiver, Sender, WeakSender, channel};
use tokio_task_pool::Pool;
use zdp::{SimpleDescReq, SimpleDescriptor, Status};
use zigbee::{Address, Endpoint};

use self::devices::Devices;
pub use self::message::Message;
use crate::discovery::attribute_discovery;
use crate::transceiver::zdp::Handle;
use crate::{MPSC_CHANNEL_SIZE, RETRY, TASK_POOL_SIZE, transceiver};

mod devices;
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
                    self.descriptor_discovered(address, descriptor).await;
                }
            }
        }
    }

    /// Discover the descriptors for the given endpoints.
    async fn discover(&mut self, address: &Address, endpoints: BTreeSet<Endpoint>) {
        self.devices.insert(
            address.clone(),
            endpoints.iter().map(|endpoint| (*endpoint, None)).collect(),
        );

        for endpoint in endpoints {
            self.tasks
                .spawn(get_descriptor(
                    address.clone(),
                    endpoint,
                    self.loopback.clone(),
                    self.zdp.clone(),
                ))
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

/// Run the per-endpoint descriptor discovery loop.
async fn get_descriptor(
    address: Address,
    endpoint: Endpoint,
    loopback: WeakSender<Message>,
    zdp: WeakSender<transceiver::zdp::Message>,
) {
    trace!("Starting discovery of descriptor for {address}:{endpoint}.");
    let short_id = address.short_id();
    let mut retries = 0;

    while RETRY.retry(&mut retries).await {
        let Some(zdp) = zdp.upgrade() else {
            trace!("Failed to upgrade ZDP sender.");
            return;
        };

        match zdp
            .communicate(short_id, SimpleDescReq::new(short_id, endpoint))
            .await
        {
            Ok(response) => {
                if response.status() == Ok(Status::Success) {
                    trace!("Got descriptor for {address}:{endpoint}.");

                    let Some(loopback) = loopback.upgrade() else {
                        return;
                    };

                    trace!("Sending descriptor for {address}:{endpoint} to loopback.");
                    loopback
                        .send(Message::DescriptorDiscovered {
                            address,
                            descriptor: response.into_descriptor(),
                        })
                        .await
                        .unwrap_or_else(|error| {
                            error!("Failed to send DescriptorsDiscovered message: {error:?}");
                        });
                    return;
                }

                warn!(
                    "Failed to get descriptor for {address}:{endpoint}: {:?}",
                    response.status()
                );
            }
            Err(error) => {
                warn!("Failed to get descriptor for {address}:{endpoint}: {error:?}");
            }
        }
    }

    error!("Failed to get descriptor for {address}:{endpoint}.");
}
