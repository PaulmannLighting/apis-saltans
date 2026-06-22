use std::collections::BTreeMap;

use log::{error, trace, warn};
use tokio::sync::mpsc::{Receiver, Sender, WeakSender, channel};
use tokio_task_pool::Pool;
use zcl::general::basic::readable::Id;
use zdp::SimpleDescriptor;
use zigbee::{Address, Application, Endpoint};

use self::devices::{Devices, DevicesExt};
pub use self::message::Message;
use crate::{
    Attributes, MPSC_CHANNEL_SIZE, RETRY, ReadAttributeResult, ReadAttributesInternal,
    TASK_POOL_SIZE, binding, transceiver,
};

mod devices;
mod endpoint_info;
mod message;

/// The attributes we want to discover.
const ATTRIBUTES: [Id; 10] = [
    Id::ZclVersion,
    Id::ApplicationVersion,
    Id::StackVersion,
    Id::HwVersion,
    Id::ManufacturerName,
    Id::ModelIdentifier,
    Id::DateCode,
    Id::PowerSource,
    Id::LocationDescription,
    Id::SwBuildId,
];

/// Actor to discover attributes on devices.
#[derive(Debug)]
pub struct AttributeDiscovery {
    inbox: Receiver<Message>,
    loopback: WeakSender<Message>,
    zcl: WeakSender<transceiver::zcl::Message>,
    binding_manager: WeakSender<binding::Message>,
    devices: Devices,
    tasks: Pool,
}

impl AttributeDiscovery {
    /// Create a new instance of `AttributeDiscovery`.
    #[must_use]
    pub fn new(
        zcl: WeakSender<transceiver::zcl::Message>,
        binding_manager: WeakSender<binding::Message>,
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
                Message::GetAttributes { address, endpoints } => {
                    self.get_attributes(&address, endpoints).await;
                }
                Message::AttributesDiscovered {
                    address,
                    application,
                    results,
                } => {
                    self.update_attributes(address, application, results).await;
                }
            }
        }
    }

    async fn get_attributes(
        &mut self,
        address: &Address,
        endpoints: BTreeMap<Endpoint, SimpleDescriptor>,
    ) {
        let application_endpoints_with_basic_cluster: Vec<_> = endpoints
            .iter()
            .filter_map(DevicesExt::application_eps_with_basic_cluster)
            .map(|(application, _)| application)
            .collect();

        *self.devices.entry(address.clone()).or_default() = endpoints
            .into_iter()
            .map(|(endpoint, descriptor)| (endpoint, descriptor.into()))
            .collect();

        for application in application_endpoints_with_basic_cluster {
            self.tasks
                .spawn(discover_attributes(
                    address.clone(),
                    application,
                    self.loopback.clone(),
                    self.zcl.clone(),
                ))
                .await
                .map_or_else(|error| error!("Failed to spawn task: {error:?}"), drop);
        }
    }

    async fn update_attributes(
        &mut self,
        address: Address,
        application: Application,
        results: Box<[ReadAttributeResult<Id>]>,
    ) {
        let Some(endpoints) = self.devices.get_mut(&address) else {
            warn!("Received attributes for unknown device: {address}");
            return;
        };

        let Some(endpoint) = endpoints.get_mut(&Endpoint::Application(application)) else {
            warn!("Received attributes for unknown endpoint: {address}:{application:#04X}");
            return;
        };

        endpoint.set_attributes(results.into());
        self.forward_device_if_complete(address).await;
    }

    async fn forward_device_if_complete(&mut self, address: Address) {
        let Some(endpoints) = self.devices.remove(&address) else {
            warn!("Received attributes for unknown device: {address}");
            return;
        };

        if endpoints
            .iter()
            .filter_map(DevicesExt::application_eps_with_basic_cluster)
            .all(|(_, endpoint_info)| endpoint_info.attributes().is_some())
        {
            let Some(binding_manager) = self.binding_manager.upgrade() else {
                trace!("Binding manager channel closed. Aborting forwarding of device: {address}.");
                return;
            };

            trace!("Forwarding device {address} to binding manager: {endpoints:?}.");
            binding_manager
                .send(binding::Message::DeviceDiscovered {
                    address,
                    endpoints: endpoints
                        .into_iter()
                        .map(|(endpoint, info)| (endpoint, info.into()))
                        .collect(),
                })
                .await
                .unwrap_or_else(|error| error!("Failed to forward device: {error:?}"));
        } else {
            trace!("Not all attributes discovered for {address}.");
            self.devices.insert(address, endpoints);
        }
    }
}

async fn discover_attributes(
    address: Address,
    application: Application,
    loopback: WeakSender<Message>,
    zcl: WeakSender<transceiver::zcl::Message>,
) {
    trace!("Starting discovery of basic attributes for {address}:{application}.");
    let mut retries = 0;

    while RETRY.retry(&mut retries).await {
        let Some(zcl) = zcl.upgrade() else {
            trace!("Failed to upgrade ZCL sender.");
            return;
        };

        match zcl
            .read_attributes(address.short_id(), application.into(), ATTRIBUTES.into())
            .await
        {
            Ok(results) => {
                let Some(loopback) = loopback.upgrade() else {
                    trace!("Failed to upgrade loopback sender.");
                    return;
                };

                trace!(
                    "Discovered basic attributes for {address}:{application}. Handing over to loopback."
                );
                loopback
                    .send(Message::AttributesDiscovered {
                        address: address.clone(),
                        application,
                        results,
                    })
                    .await
                    .unwrap();
                return;
            }
            Err(error) => {
                error!(
                    "Failed to discover basic attributes for {address}:{application:#04X}: {error}"
                );
            }
        }
    }

    error!("Failed to discover basic attributes for {address}:{application:#04X}.");
}
