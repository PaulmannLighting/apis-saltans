use std::collections::BTreeMap;

use log::{error, info, trace, warn};
use tokio::sync::mpsc::{Receiver, Sender, WeakSender, channel};
use tokio_task_pool::Pool;
use zb_core::{Application, Endpoint, FullAddress};
use zb_zcl::basic::Id;

use self::discovery_task::DiscoveryTask;
pub use self::incoming_device::IncomingDevice;
pub use self::message::Message;
pub use self::outgoing_device::OutgoingDevice;
use self::outgoing_device::{Devices, DevicesExt};
use crate::{
    Attributes, MPSC_CHANNEL_SIZE, ReadAttributeResult, TASK_POOL_SIZE, binding, transceiver,
};

mod discovery_task;
mod endpoint_info;
mod incoming_device;
mod message;
mod outgoing_device;

/// Actor to discover attributes on devices.
#[derive(Debug)]
pub struct AttributeDiscovery {
    inbox: Receiver<Message>,
    loopback: WeakSender<Message>,
    zcl: WeakSender<transceiver::zcl::Message>,
    binding_manager: Sender<binding::Message>,
    devices: Devices,
    tasks: Pool,
}

impl AttributeDiscovery {
    /// Create a new instance of `AttributeDiscovery`.
    #[must_use]
    pub fn new(
        zcl: WeakSender<transceiver::zcl::Message>,
        binding_manager: Sender<binding::Message>,
    ) -> (Self, Sender<Message>) {
        let (tx, rx) = channel(MPSC_CHANNEL_SIZE);
        let instance = Self {
            inbox: rx,
            loopback: tx.downgrade(),
            zcl,
            binding_manager,
            devices: BTreeMap::new(),
            tasks: Pool::bounded(TASK_POOL_SIZE),
        };
        (instance, tx)
    }

    /// Run the actor.
    pub async fn run(mut self) {
        while let Some(message) = self.inbox.recv().await {
            match message {
                Message::GetAttributes(device) => {
                    self.get_attributes(device).await;
                }
                Message::AttributesDiscovered {
                    address,
                    application,
                    results,
                } => {
                    self.update_attributes(address, application, results).await;
                }
                Message::DiscoveryFailed(address) => {
                    if self.devices.remove(&address.ieee_address()).is_some() {
                        trace!("Removed failed discovery of: {address}");
                    }
                }
            }
        }
    }

    async fn get_attributes(&mut self, device: IncomingDevice) {
        if self.devices.contains_key(&device.address.ieee_address()) {
            trace!("Discovery for {device} already in progress.");
            return;
        }

        let Some(loopback) = self.loopback.upgrade() else {
            warn!("Failed to upgrade loopback channel.");
            return;
        };

        let Some(zcl) = self.zcl.upgrade() else {
            warn!("Failed to upgrade ZCL channel.");
            return;
        };

        let device = self
            .devices
            .entry(device.address.ieee_address())
            .or_insert_with(|| device.into());

        for application in device
            .endpoints
            .iter()
            .filter_map(DevicesExt::application_eps_with_basic_cluster)
            .map(|(application, _)| application)
        {
            self.tasks
                .spawn(
                    DiscoveryTask::new(device.address, application, loopback.clone(), zcl.clone())
                        .run(),
                )
                .await
                .map_or_else(|error| error!("Failed to spawn task: {error:?}"), drop);
        }
    }

    async fn update_attributes(
        &mut self,
        address: FullAddress,
        application: Application,
        results: Box<[ReadAttributeResult<Id>]>,
    ) {
        let Some(mut device) = self.devices.remove(&address.ieee_address()) else {
            warn!("Received attributes for unknown device: {address}");
            return;
        };

        let Some(endpoint) = device
            .endpoints
            .get_mut(&Endpoint::Application(application))
        else {
            warn!("Received attributes for unknown endpoint: {address}:{application:#04X}");
            self.devices.insert(address.ieee_address(), device);
            return;
        };

        endpoint.set_attributes(results.into());

        if !device
            .endpoints
            .iter()
            .filter_map(DevicesExt::application_eps_with_basic_cluster)
            .all(|(_, endpoint_info)| endpoint_info.attributes().is_some())
        {
            trace!("Not all attributes discovered for {address}.");
            self.devices.insert(address.ieee_address(), device);
            return;
        }

        info!("All attributes discovered for {address}.");
        trace!("Forwarding device {address} to binding manager.");
        self.binding_manager
            .send(binding::Message::DeviceDiscovered(Box::new(device.into())))
            .await
            .unwrap_or_else(|error| error!("Failed to forward device: {error:?}"));
    }
}
