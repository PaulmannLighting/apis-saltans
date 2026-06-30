use std::collections::BTreeSet;

use log::{error, trace, warn};
use tokio::sync::mpsc::{Receiver, Sender, WeakSender, channel};
use tokio_task_pool::Pool;
use zigbee::Address;

pub use self::message::Message;
use crate::discovery::descriptor_discovery::discovery_task::DiscoveryTask;
use crate::discovery::endpoint_discovery::{self, Device};
use crate::{MPSC_CHANNEL_SIZE, TASK_POOL_SIZE, transceiver};

mod discovery_task;
mod message;

#[derive(Debug)]
pub struct DescriptorDiscovery {
    inbox: Receiver<Message>,
    loopback: Sender<Message>,
    zdp: WeakSender<transceiver::zdp::Message>,
    endpoint_discovery: Sender<endpoint_discovery::Message>,
    pending: BTreeSet<Address>,
    tasks: Pool,
}

impl DescriptorDiscovery {
    /// Create a new instance of `DescriptorDiscovery`.
    #[must_use]
    pub fn new(
        zdp: WeakSender<transceiver::zdp::Message>,
        endpoint_discovery: Sender<endpoint_discovery::Message>,
    ) -> (Self, Sender<Message>) {
        let (tx, rx) = channel(MPSC_CHANNEL_SIZE);

        let instance = Self {
            inbox: rx,
            loopback: tx.clone(),
            zdp,
            endpoint_discovery,
            pending: BTreeSet::new(),
            tasks: Pool::bounded(TASK_POOL_SIZE),
        };

        (instance, tx)
    }

    /// Run the actor.
    pub async fn run(mut self) {
        while let Some(message) = self.inbox.recv().await {
            match message {
                Message::Discover(address) => {
                    self.start_discovery(address).await;
                }
                Message::DescriptorDiscovered {
                    address,
                    descriptor,
                } => {
                    if !self.pending.remove(&address) {
                        warn!("Received descriptor for unknown device: {address}");
                    }

                    self.endpoint_discovery
                        .send(endpoint_discovery::Message::Discover(Device::new(
                            address,
                            *descriptor,
                        )))
                        .await
                        .unwrap_or_else(|error| {
                            error!("Failed to forward to endpoint discovery: {error:?}")
                        })
                }
                Message::DiscoveryFailed(address) => {
                    if !self.pending.remove(&address) {
                        warn!("Received discovery failure for unknown device: {address}");
                    }
                }
            }
        }
    }

    async fn start_discovery(&mut self, address: Address) {
        if self.pending.contains(&address) {
            trace!("Already discovering descriptors for {address}");
            return;
        }

        let Some(zdp) = self.zdp.upgrade() else {
            trace!("Failed to upgrade ZDP sender");
            return;
        };

        self.pending.insert(address.clone());
        self.tasks
            .spawn(DiscoveryTask::new(address, self.loopback.clone(), zdp).run())
            .await
            .map_or_else(
                |error| {
                    error!("Failed to spawn task: {error:?}");
                },
                drop,
            );
    }
}
