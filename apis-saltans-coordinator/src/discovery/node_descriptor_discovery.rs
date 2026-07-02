use std::collections::BTreeMap;

use apis_saltans_core::Address;
use log::{error, trace, warn};
use tokio::sync::mpsc::{Receiver, Sender, WeakSender, channel};
use tokio::task::JoinHandle;
use tokio_task_pool::{Error, Pool};

pub use self::message::Message;
use crate::discovery::endpoint_discovery::{self, Device};
use crate::discovery::node_descriptor_discovery::discovery_task::DiscoveryTask;
use crate::{MPSC_CHANNEL_SIZE, TASK_POOL_SIZE, transceiver};

mod discovery_task;
mod message;

#[derive(Debug)]
pub struct NodeDescriptorDiscovery {
    inbox: Receiver<Message>,
    loopback: Sender<Message>,
    zdp: WeakSender<transceiver::zdp::Message>,
    endpoint_discovery: Sender<endpoint_discovery::Message>,
    pending: BTreeMap<Address, JoinHandle<Result<(), Error>>>,
    tasks: Pool,
}

impl NodeDescriptorDiscovery {
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
            pending: BTreeMap::new(),
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
                    if self.pending.remove(&address).is_none() {
                        warn!("Received descriptor for unknown device: {address}");
                    }

                    self.endpoint_discovery
                        .send(endpoint_discovery::Message::Discover(Device::new(
                            address,
                            *descriptor,
                        )))
                        .await
                        .unwrap_or_else(|error| {
                            error!("Failed to forward to endpoint discovery: {error:?}");
                        });
                }
                Message::DiscoveryFailed(address) => {
                    if self.pending.remove(&address).is_none() {
                        warn!("Received discovery failure for unknown device: {address}");
                    }
                }
            }
        }
    }

    async fn start_discovery(&mut self, address: Address) {
        if let Some(join_handle) = self.pending.remove(&address) {
            trace!("Terminating already running discovery if node descriptors for {address}");
            join_handle.abort();
        }

        let Some(zdp) = self.zdp.upgrade() else {
            trace!("Failed to upgrade ZDP sender");
            return;
        };

        if let Ok(join_handle) = self
            .tasks
            .spawn(DiscoveryTask::new(address.clone(), self.loopback.clone(), zdp).run())
            .await
            .inspect_err(|error| {
                error!("Failed to spawn task: {error:?}");
            })
        {
            self.pending.insert(address, join_handle);
        }
    }
}
