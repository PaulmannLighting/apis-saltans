use std::collections::{BTreeMap, BTreeSet};

use log::{error, trace, warn};
use tokio::sync::mpsc::{Receiver, Sender, WeakSender, channel};
use tokio_task_pool::Pool;
use zdp::{SimpleDescReq, SimpleDescriptor, Status};
use zigbee::{Address, Endpoint};

pub use self::message::Message;
use crate::discovery::attribute_discovery;
use crate::transceiver::zdp::Handle;
use crate::{RETRY, TASK_POOL_SIZE, transceiver};

mod message;

/// Actor to discover descriptors on devices.
#[derive(Debug)]
pub struct DescriptorDiscovery {
    inbox: Receiver<Message>,
    loopback: WeakSender<Message>,
    zdp: WeakSender<transceiver::zdp::Message>,
    attribute_discovery: Sender<attribute_discovery::Message>,
    descriptors: BTreeMap<Address, BTreeMap<Endpoint, Option<SimpleDescriptor>>>,
    tasks: Pool,
}

impl DescriptorDiscovery {
    /// Create a new instance of `DescriptorDiscovery`.
    #[must_use]
    pub fn new(
        buffer: usize,
        zdp: WeakSender<transceiver::zdp::Message>,
        attribute_discovery: Sender<attribute_discovery::Message>,
    ) -> (Self, Sender<Message>) {
        let (tx, rx) = channel(buffer);
        let instance = Self {
            inbox: rx,
            loopback: tx.downgrade(),
            zdp,
            attribute_discovery,
            descriptors: BTreeMap::new(),
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
                Message::DescriptorsDiscovered {
                    address,
                    descriptors,
                } => {
                    self.descriptors_discovered(address, descriptors).await;
                }
            }
        }
    }

    /// Discover the descriptors for the given endpoints.
    async fn discover(&mut self, address: &Address, endpoints: BTreeSet<Endpoint>) {
        self.descriptors.insert(
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
                .map_or_else(drop, |error| {
                    error!("Failed to spawn task: {error:?}");
                });
        }
    }

    /// Update the descriptor map with the newly discovered descriptors.
    async fn descriptors_discovered(
        &mut self,
        address: Address,
        descriptors: Box<[SimpleDescriptor]>,
    ) {
        let device = self.descriptors.entry(address.clone()).or_default();

        for descriptor in descriptors {
            device.insert(descriptor.endpoint(), Some(descriptor));
        }

        self.forward_descriptors_if_complete(address).await;
    }

    /// Forward the descriptors if all have been discovered.
    async fn forward_descriptors_if_complete(&mut self, address: Address) {
        let Some(descriptors) = self.descriptors.get(&address) else {
            error!("Got descriptors for {address:?} before we discovered them.");
            return;
        };

        if descriptors.values().any(Option::is_none) {
            trace!("Not all descriptors for {address:?} discovered.");
            return;
        }

        trace!("All descriptors for {address:?} discovered.");

        let endpoints = self
            .descriptors
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
    let short_id = address.short_id();
    let mut retries = 0;

    while RETRY.retry(&mut retries).await {
        let Some(zdp) = zdp.upgrade() else {
            return;
        };

        match zdp
            .communicate(short_id, SimpleDescReq::new(short_id, endpoint))
            .await
        {
            Ok(response) => {
                if response.status() == Ok(Status::Success) {
                    trace!("Got descriptor for {address:?} on endpoint {endpoint:?}");

                    let Some(loopback) = loopback.upgrade() else {
                        return;
                    };

                    loopback
                        .send(Message::DescriptorsDiscovered {
                            address,
                            descriptors: response.into_descriptors(),
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
