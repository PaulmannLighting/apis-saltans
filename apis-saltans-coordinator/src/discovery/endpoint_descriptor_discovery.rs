use std::collections::BTreeMap;

use apis_saltans_core::Address;
use apis_saltans_zdp::SimpleDescriptor;
use log::{error, info, trace, warn};
use tokio::sync::mpsc::{Receiver, Sender, WeakSender, channel};
use tokio_task_pool::Pool;

pub use self::device::Device;
use self::devices::Devices;
pub use self::message::Message;
use crate::discovery::attribute_discovery;
use crate::discovery::endpoint_descriptor_discovery::discovery_task::DiscoveryTask;
use crate::{MPSC_CHANNEL_SIZE, TASK_POOL_SIZE, transceiver};

mod device;
mod devices;
mod discovery_task;
mod message;

/// Actor to discover descriptors on devices.
#[derive(Debug)]
pub struct EndpointDescriptorDiscovery {
    inbox: Receiver<Message>,
    loopback: WeakSender<Message>,
    zdp: WeakSender<transceiver::zdp::Message>,
    attribute_discovery: Sender<attribute_discovery::Message>,
    devices: Devices,
    tasks: Pool,
}

impl EndpointDescriptorDiscovery {
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
                Message::Discover(device) => {
                    self.discover(device).await;
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
    async fn discover(&mut self, device: Device) {
        if self.devices.contains_key(&device.address) {
            trace!("Already discovering descriptors for {device}");
            return;
        }

        let device = self
            .devices
            .entry(device.address)
            .or_insert_with(|| device.into());

        let Some(loopback) = self.loopback.upgrade() else {
            warn!("Failed to upgrade loopback channel.");
            return;
        };

        let Some(zdp) = self.zdp.upgrade() else {
            warn!("Failed to upgrade ZDP channel.");
            return;
        };

        for endpoint in device.endpoints.keys() {
            self.tasks
                .spawn(
                    DiscoveryTask::new(device.address, *endpoint, loopback.clone(), zdp.clone())
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
        let Some(mut device) = self.devices.remove(&address) else {
            warn!("Discarding endpoint descriptor for {address} before we discovered them.");
            return;
        };

        trace!("Discovered endpoint descriptor for {address}: {descriptor:?}");
        device
            .endpoints
            .insert(descriptor.endpoint(), Some(descriptor));

        if device.endpoints.values().any(Option::is_none) {
            trace!("Not all descriptors for {address} discovered.");
            self.devices.insert(address, device);
            return;
        }

        info!("All endpoint descriptors for {address} discovered.");

        let endpoints = device
            .endpoints
            .into_iter()
            .filter_map(|(endpoint, descriptor)| {
                descriptor.map(|descriptor| (endpoint, descriptor))
            })
            .collect();

        trace!("Forwarding descriptors for {address} to attribute discovery: {endpoints:?}");
        self.attribute_discovery
            .send(attribute_discovery::Message::GetAttributes(
                attribute_discovery::Device::new(device.address, device.descriptor, endpoints),
            ))
            .await
            .unwrap_or_else(|error| error!("Failed to send GetAttributes message: {error:?}"));
    }
}
