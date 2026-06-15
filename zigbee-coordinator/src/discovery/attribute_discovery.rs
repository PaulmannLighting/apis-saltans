use std::collections::BTreeMap;
use std::time::Duration;

use log::{error, warn};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use tokio::time::sleep;
use zcl::general::basic::readable::Id;
use zdp::SimpleDescriptor;
use zigbee::{Address, Application, Endpoint};

use self::application_endpoints_with_basic_cluster::ApplicationEndpointsWithBasicCluster;
pub use self::endpoint_info::EndpointInfo;
pub use self::message::Message;
use crate::{ReadAttributeResult, ReadAttributes, binding, network_manager, transceiver};

mod application_endpoints_with_basic_cluster;
mod attributes;
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
    loopback: Sender<Message>,
    zcl: Sender<transceiver::zcl::Message>,
    binding_manager: Sender<binding::Message>,
    max_retries: usize,
    retry_delay: Duration,
    attributes: BTreeMap<Address, BTreeMap<Endpoint, EndpointInfo>>,
}

impl AttributeDiscovery {
    /// Create a new instance of `AttributeDiscovery`.
    #[must_use]
    pub fn new(
        buffer: usize,
        zcl: Sender<transceiver::zcl::Message>,
        binding_manager: Sender<binding::Message>,
        max_retries: usize,
        retry_delay: Duration,
    ) -> (Self, Sender<Message>) {
        let (tx, rx) = channel(buffer);
        let instance = Self {
            inbox: rx,
            loopback: tx.clone(),
            zcl,
            binding_manager,
            max_retries,
            retry_delay,
            attributes: BTreeMap::new(),
        };
        (instance, tx)
    }

    /// Run the actor.
    pub async fn run(mut self) {
        while let Some(message) = self.inbox.recv().await {
            match message {
                Message::GetAttributes { address, endpoints } => {
                    self.get_attributes(address, endpoints).await
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
        address: Address,
        endpoints: BTreeMap<Endpoint, SimpleDescriptor>,
    ) {
        let application_endpoints_with_basic_cluster: Vec<_> = endpoints
            .iter()
            .filter_map(ApplicationEndpointsWithBasicCluster::filter)
            .map(|(application, _)| application)
            .collect();

        *self.attributes.entry(address.clone()).or_default() = endpoints
            .into_iter()
            .map(|(endpoint, descriptor)| (endpoint, descriptor.into()))
            .collect();

        for application in application_endpoints_with_basic_cluster {
            spawn(discover_attributes(
                address.clone(),
                application,
                self.loopback.clone(),
                self.zcl.clone(),
                self.max_retries,
                self.retry_delay,
            ));
        }
    }

    async fn update_attributes(
        &mut self,
        address: Address,
        application: Application,
        results: Box<[ReadAttributeResult<Id>]>,
    ) {
        let Some(endpoints) = self.attributes.get_mut(&address) else {
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
        let Some(endpoints) = self.attributes.remove(&address) else {
            return;
        };

        if endpoints
            .iter()
            .filter_map(ApplicationEndpointsWithBasicCluster::filter)
            .all(|(_, endpoint_info)| endpoint_info.attributes().is_some())
        {
            self.binding_manager
                .send(binding::Message::DeviceDiscovered { address, endpoints })
                .await
                .unwrap_or_else(|error| error!("Failed to forward device: {error:?}"))
        }
    }
}

async fn discover_attributes(
    address: Address,
    application: Application,
    loopback: Sender<Message>,
    zcl: Sender<transceiver::zcl::Message>,
    max_retries: usize,
    retry_delay: Duration,
) {
    let mut retries = 0;

    loop {
        if retries > max_retries {
            error!(
                "Failed to discover basic attributes for {address}:{application:#04X} after {max_retries} retries. Giving up."
            );
            return;
        }

        if retries > 0 {
            sleep(retry_delay).await;
        }

        retries += 1;

        match zcl
            .read_attributes(address.clone(), application.into(), ATTRIBUTES.into())
            .await
        {
            Ok(results) => {
                loopback
                    .send(Message::AttributesDiscovered {
                        address: address.clone(),
                        application,
                        results,
                    })
                    .await
                    .unwrap();
            }
            Err(error) => {
                error!(
                    "Failed to discover basic attributes for {address}:{application:#04X}: {error}"
                );
            }
        }
    }
}
