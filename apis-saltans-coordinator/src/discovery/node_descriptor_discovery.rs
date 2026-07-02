use apis_saltans_core::Address;
use log::{error, trace};
use tokio::sync::mpsc::{Receiver, Sender, WeakSender, channel};
use tokio_task_pool::Pool;

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
                    error!("Received discovery failed: {address}");
                }
            }
        }
    }

    async fn start_discovery(&self, address: Address) {
        let Some(zdp) = self.zdp.upgrade() else {
            trace!("Failed to upgrade ZDP sender");
            return;
        };

        self.tasks
            .spawn(DiscoveryTask::new(address.clone(), self.loopback.clone(), zdp).run())
            .await
            .map_or_else(
                |error| {
                    error!("Failed to spawn task: {error:?}");
                },
                drop,
            );
    }
}
