use std::collections::BTreeMap;
use std::time::Duration;

use const_env::env_item;
use log::{error, trace, warn};
use tokio::sync::mpsc::{Receiver, Sender, WeakSender, channel};
use tokio_task_pool::Pool;
use zcl::general::basic::readable::Id;
use zdp::SimpleDescriptor;
use zigbee::{Address, Application, Endpoint};

use self::devices::{Devices, DevicesExt};
pub use self::message::Message;
use crate::discovery::attribute_discovery::endpoint_info::EndpointInfo;
use crate::timeout::Timeout;
use crate::{
    Attributes, MPSC_CHANNEL_SIZE, RETRY, ReadAttributeResult, ReadAttributesInternal,
    TASK_POOL_SIZE, binding, transceiver,
};

mod devices;
mod endpoint_info;
mod message;

#[env_item("ZIGBEE_COORDINATOR_ATTRIBUTE_DISCOVERY_TIMEOUT_SECS")]
const TIMEOUT_SECS: u64 = 5;
const TIMEOUT: Duration = Duration::from_secs(TIMEOUT_SECS);

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
                Message::DiscoveryFailed { address } => {
                    if self.devices.remove(&address).is_some() {
                        trace!("Removed failed discovery of: {address}");
                    }
                }
            }
        }
    }

    async fn get_attributes(
        &mut self,
        address: &Address,
        endpoints: BTreeMap<Endpoint, SimpleDescriptor>,
    ) {
        if self.devices.contains_key(address) {
            trace!("Discovery for {address} already in progress.");
            return;
        }

        let application_endpoints_with_basic_cluster: Vec<_> = endpoints
            .iter()
            .filter_map(DevicesExt::application_eps_with_basic_cluster)
            .map(|(application, _)| application)
            .collect();

        *self.devices.entry(address.clone()).or_default() = endpoints
            .into_iter()
            .map(|(endpoint, descriptor)| (endpoint, descriptor.into()))
            .collect();

        let Some(loopback) = self.loopback.upgrade() else {
            warn!("Failed to upgrade loopback channel.");
            return;
        };

        let Some(zcl) = self.zcl.upgrade() else {
            warn!("Failed to upgrade ZCL channel.");
            return;
        };

        for application in application_endpoints_with_basic_cluster {
            self.tasks
                .spawn(discover_attributes(
                    address.clone(),
                    application,
                    loopback.clone(),
                    zcl.clone(),
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
        let Some(mut endpoints) = self.devices.remove(&address) else {
            warn!("Received attributes for unknown device: {address}");
            return;
        };

        let Some(endpoint) = endpoints.get_mut(&Endpoint::Application(application)) else {
            warn!("Received attributes for unknown endpoint: {address}:{application:#04X}");
            self.devices.insert(address, endpoints);
            return;
        };

        endpoint.set_attributes(results.into());
        self.forward_device_if_complete(address, endpoints).await;
    }

    async fn forward_device_if_complete(
        &mut self,
        address: Address,
        endpoints: BTreeMap<Endpoint, EndpointInfo>,
    ) {
        if endpoints
            .iter()
            .filter_map(DevicesExt::application_eps_with_basic_cluster)
            .all(|(_, endpoint_info)| endpoint_info.attributes().is_some())
        {
            trace!("All attributes discovered for {address}: {endpoints:?}.");
            self.forward_device(address, endpoints).await;
        } else {
            trace!("Not all attributes discovered for {address}.");
            self.devices.insert(address, endpoints);
        }
    }

    async fn forward_device(&self, address: Address, endpoints: BTreeMap<Endpoint, EndpointInfo>) {
        trace!("Forwarding device {address} to binding manager.");
        self.binding_manager
            .send(binding::Message::DeviceDiscovered {
                address,
                endpoints: endpoints
                    .into_iter()
                    .map(|(endpoint, info)| (endpoint, info.into()))
                    .collect(),
            })
            .await
            .unwrap_or_else(|error| error!("Failed to forward device: {error:?}"));
    }
}

async fn discover_attributes(
    address: Address,
    application: Application,
    loopback: Sender<Message>,
    zcl: Sender<transceiver::zcl::Message>,
) {
    trace!("Starting discovery of basic attributes for {address}:{application}.");
    let mut retries = 0;

    while RETRY.retry(&mut retries).await {
        match zcl
            .read_attributes_one_by_one(address.short_id(), application, ATTRIBUTES.into())
            .timeout(TIMEOUT * u32::try_from(ATTRIBUTES.len()).unwrap_or(u32::MAX))
            .await
        {
            Ok(results) => {
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
                    .unwrap_or_else(|error| {
                        error!("Failed to send AttributesDiscovered message: {error:?}");
                    });
                return;
            }
            Err(error) => {
                error!("Failed to read attributes for {address}:{application:#04X}: {error}");
            }
        }
    }

    error!("Failed to discover basic attributes for {address}:{application:#04X}.");
    loopback
        .send(Message::DiscoveryFailed { address })
        .await
        .unwrap_or_else(|error| error!("Failed to send DiscoveryFailed message: {error:?}"));
}
