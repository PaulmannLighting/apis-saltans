use std::collections::{BTreeMap, BTreeSet};

use log::{error, trace, warn};
use tokio::sync::mpsc::{Receiver, Sender, WeakSender, channel};
use tokio_task_pool::Pool;
use zigbee::Address;

pub use self::device::Device;
pub use self::message::Message;
use super::endpoint_descriptor_discovery;
use crate::discovery::endpoint_discovery::discovery_task::DiscoveryTask;
use crate::{MPSC_CHANNEL_SIZE, TASK_POOL_SIZE, transceiver};

mod device;
mod discovery_task;
mod message;

/// Actor to discover endpoints on devices.
#[derive(Debug)]
pub struct EndpointDiscovery {
    inbox: Receiver<Message>,
    loopback: Sender<Message>,
    zdp: WeakSender<transceiver::zdp::Message>,
    descriptor_discovery: Sender<endpoint_descriptor_discovery::Message>,
    tasks: Pool,
    pending: BTreeMap<Address, Device>,
}

impl EndpointDiscovery {
    /// Create a new instance of `EndpointDiscovery`.
    #[must_use]
    pub fn new(
        zdp: WeakSender<transceiver::zdp::Message>,
        descriptor_discovery: Sender<endpoint_descriptor_discovery::Message>,
    ) -> (Self, Sender<Message>) {
        let (tx, rx) = channel(MPSC_CHANNEL_SIZE);

        let instance = Self {
            inbox: rx,
            loopback: tx.clone(),
            zdp,
            descriptor_discovery,
            tasks: Pool::bounded(TASK_POOL_SIZE),
            pending: BTreeMap::new(),
        };

        (instance, tx)
    }

    /// Run the actor.
    pub async fn run(mut self) {
        while let Some(message) = self.inbox.recv().await {
            match message {
                Message::Discover(device) => {
                    self.discover_endpoints(device).await;
                }
                Message::Discovered { address, endpoints } => {
                    let Some(device) = self.pending.remove(&address) else {
                        warn!("Received Discovered message for unknown device: {address}");
                        continue;
                    };

                    self.descriptor_discovery
                        .send(endpoint_descriptor_discovery::Message::Discover(
                            endpoint_descriptor_discovery::Device::new(
                                device.address,
                                device.descriptor,
                                endpoints,
                            ),
                        ))
                        .await
                        .unwrap_or_else(|error| {
                            error!("Failed to forward to descriptor discovery: {error:?}");
                        });
                }
                Message::DiscoveryFailed(address) => {
                    if self.pending.remove(&address).is_none() {
                        warn!("Received DiscoveryFailed message for unknown device: {address}");
                    }
                }
            }
        }
    }

    /// Discover endpoints on the given device in a separate task.
    async fn discover_endpoints(&self, device: Device) {
        if self.pending.contains_key(&device.address) {
            trace!("Already discovering endpoints for {}", device.address);
            return;
        }

        let Some(zdp) = self.zdp.upgrade() else {
            warn!("Failed to upgrade ZDP sender");
            return;
        };

        self.tasks
            .spawn(DiscoveryTask::new(device.address, zdp, self.loopback.clone()).run())
            .await
            .map_or_else(
                |error| {
                    error!("Failed to spawn task: {error:?}");
                },
                drop,
            );
    }
}
