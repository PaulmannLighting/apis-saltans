use std::collections::{BTreeMap, BTreeSet};
use std::time::Duration;

use log::{error, trace, warn};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use tokio::time::sleep;
use zdp::{SimpleDescReq, SimpleDescriptor, Status};
use zigbee::{Address, Endpoint};

pub use self::message::Message;
use crate::discovery::attribute_discovery;
use crate::transceiver;
use crate::transceiver::zdp::Handle;

mod message;

/// Actor to discover descriptors on devices.
#[derive(Debug)]
pub struct DescriptorDiscovery {
    inbox: Receiver<Message>,
    loopback: Sender<Message>,
    zdp: Sender<transceiver::zdp::Message>,
    attribute_discovery: Sender<attribute_discovery::Message>,
    max_retries: usize,
    retry_delay: Duration,
    descriptors: BTreeMap<Address, BTreeMap<Endpoint, Option<SimpleDescriptor>>>,
}

impl DescriptorDiscovery {
    /// Create a new instance of `DescriptorDiscovery`.
    #[must_use]
    pub fn new(
        buffer: usize,
        zdp: Sender<transceiver::zdp::Message>,
        attribute_discovery: Sender<attribute_discovery::Message>,
        max_retries: usize,
        retry_delay: Duration,
    ) -> (Self, Sender<Message>) {
        let (tx, rx) = channel(buffer);
        let instance = Self {
            inbox: rx,
            loopback: tx.clone(),
            zdp,
            attribute_discovery,
            max_retries,
            retry_delay,
            descriptors: BTreeMap::new(),
        };
        (instance, tx)
    }

    /// Run the actor.
    pub async fn run(mut self) {
        while let Some(message) = self.inbox.recv().await {
            match message {
                Message::Discover { address, endpoints } => {
                    self.discover(address, endpoints).await;
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
    async fn discover(&mut self, address: Address, endpoints: BTreeSet<Endpoint>) {
        self.descriptors.insert(
            address.clone(),
            endpoints.iter().map(|endpoint| (*endpoint, None)).collect(),
        );

        for endpoint in endpoints {
            spawn(get_descriptor(
                address.clone(),
                endpoint,
                self.loopback.clone(),
                self.zdp.clone(),
                self.max_retries,
                self.retry_delay,
            ));
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

        if descriptors.values().any(|descriptor| descriptor.is_none()) {
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
    loopback: Sender<Message>,
    zdp: Sender<transceiver::zdp::Message>,
    max_retries: usize,
    retry_delay: Duration,
) {
    let short_id = address.short_id();
    let mut retries = 0;

    loop {
        if retries > max_retries {
            error!(
                "Failed to get descriptor for {address}:{endpoint} after {max_retries} retries. Giving up."
            );
            return;
        }

        if retries > 0 {
            sleep(retry_delay).await;
        }

        retries += 1;

        match zdp
            .communicate(short_id, SimpleDescReq::new(short_id, endpoint))
            .await
        {
            Ok(response) => {
                if let Ok(Status::Success) = response.status() {
                    trace!("Got descriptor for {address:?} on endpoint {endpoint:?}");

                    loopback
                        .send(Message::DescriptorsDiscovered {
                            address,
                            descriptors: response.into_descriptors(),
                        })
                        .await
                        .unwrap_or_else(|error| {
                            error!("Failed to send DescriptorsDiscovered message: {error:?}")
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
}
