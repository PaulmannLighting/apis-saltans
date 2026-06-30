use std::collections::BTreeSet;

use log::{error, warn};
use tokio::sync::mpsc::{Receiver, Sender, WeakSender, channel};
use tokio_task_pool::Pool;
use zigbee::Address;

pub use self::message::Message;
use super::descriptor_discovery;
use crate::discovery::endpoint_discovery::discovery_task::DiscoveryTask;
use crate::{MPSC_CHANNEL_SIZE, TASK_POOL_SIZE, transceiver};

mod discovery_task;
mod message;

/// Actor to discover endpoints on devices.
#[derive(Debug)]
pub struct EndpointDiscovery {
    inbox: Receiver<Message>,
    loopback: Sender<Message>,
    zdp: WeakSender<transceiver::zdp::Message>,
    descriptor_discovery: Sender<descriptor_discovery::Message>,
    tasks: Pool,
    pending: BTreeSet<Address>,
}

impl EndpointDiscovery {
    /// Create a new instance of `EndpointDiscovery`.
    #[must_use]
    pub fn new(
        zdp: WeakSender<transceiver::zdp::Message>,
        descriptor_discovery: Sender<descriptor_discovery::Message>,
    ) -> (Self, Sender<Message>) {
        let (tx, rx) = channel(MPSC_CHANNEL_SIZE);

        let instance = Self {
            inbox: rx,
            loopback: tx.clone(),
            zdp,
            descriptor_discovery,
            tasks: Pool::bounded(TASK_POOL_SIZE),
            pending: BTreeSet::new(),
        };

        (instance, tx)
    }

    /// Run the actor.
    pub async fn run(mut self) {
        while let Some(message) = self.inbox.recv().await {
            match message {
                Message::Discover(address) => {
                    self.discover_endpoints(address).await;
                }
                Message::Discovered { address, endpoints } => {
                    if !self.pending.remove(&address) {
                        warn!("Received Discovered message for unknown device: {address:?}");
                    }

                    self.descriptor_discovery
                        .send(descriptor_discovery::Message::Discover { address, endpoints })
                        .await
                        .unwrap_or_else(|error| {
                            error!("Failed to forward to descriptor discovery: {error:?}");
                        });
                }
                Message::DiscoveryFailed(address) => {
                    if !self.pending.remove(&address) {
                        warn!("Received DiscoveryFailed message for unknown device: {address:?}");
                    }
                }
            }
        }
    }

    /// Discover endpoints on the given device in a separate task.
    async fn discover_endpoints(&self, address: Address) {
        let Some(zdp) = self.zdp.upgrade() else {
            warn!("Failed to upgrade ZDP sender");
            return;
        };

        self.tasks
            .spawn(DiscoveryTask::new(address, zdp, self.loopback.clone()).run())
            .await
            .map_or_else(
                |error| {
                    error!("Failed to spawn task: {error:?}");
                },
                drop,
            );
    }
}
