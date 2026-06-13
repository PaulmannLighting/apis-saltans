use std::collections::{BTreeMap, BTreeSet};
use std::time::Duration;

use log::{error, info, trace, warn};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use tokio::time::sleep;
use zdp::{ActiveEpReq, ActiveEpRsp, SimpleDescReq, SimpleDescRsp, Status};
use zigbee::{Address, Application};
use zigbee_hw::Event;

use self::endpoint::Endpoint;
pub use self::message::Message;
use crate::discovery::endpoint::Attributes;
use crate::transceiver::zdp::Handle;
use crate::{Error, binding, transceiver};

mod device_discovery;
mod endpoint;
mod message;

const RESCHEDULE_DELAY: Duration = Duration::from_secs(30);

/// The device discovery actor.
#[derive(Debug)]
pub struct Actor {
    inbox: Receiver<Message>,
    loopback: Sender<Message>,
    zcl_transceiver: Sender<transceiver::zcl::Message>,
    zdp_transceiver: Sender<transceiver::zdp::Message>,
    binding_manager: Sender<binding::Message>,
    // Discovery stages:
    discovered_devices: BTreeSet<Address>,
    device_endpoints: BTreeMap<Address, BTreeSet<Application>>,
    discovered_endpoints: BTreeMap<Address, BTreeMap<Application, Endpoint>>,
}

impl Actor {
    /// Create a new discovery actor.
    #[must_use]
    pub fn new(
        channel_size: usize,
        zcl_transceiver: Sender<transceiver::zcl::Message>,
        zdp_transceiver: Sender<transceiver::zdp::Message>,
        binding_manager: Sender<binding::Message>,
    ) -> (Self, Sender<Message>) {
        let (tx, rx) = channel(channel_size);

        let instance = Self {
            inbox: rx,
            loopback: tx.clone(),
            zcl_transceiver,
            zdp_transceiver,
            binding_manager,
            discovered_devices: BTreeSet::new(),
            device_endpoints: BTreeMap::new(),
            discovered_endpoints: BTreeMap::new(),
        };

        (instance, tx)
    }

    /// Run the discovery actor.
    pub async fn run(mut self) {
        while let Some(message) = self.inbox.recv().await {
            match message {
                Message::Event(event) => self.handle_event(event),
                Message::ActiveEpRsp { address, result } => {
                    self.handle_active_ep_rsp(address, result);
                }
                Message::SimpleDescRsp { address, result } => {
                    self.handle_simple_desc_rsp(address, result);
                }
                Message::Attributes {
                    address,
                    endpoint,
                    cluster_id,
                    attributes,
                } => {
                    self.update_attributes(address, endpoint, cluster_id, attributes);
                }
            }
        }
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::DeviceJoined(address) => self.handle_join(address),
            Event::DeviceRejoined { address, secured } => {
                info!("Device rejoined: {address:?}, secured: {secured}");
                self.handle_join(address);
            }
            _ => trace!("Unhandled event: {event:?}"),
        }
    }
}

/// Stage 1: Device discovery.
impl Actor {
    fn handle_join(&mut self, address: Address) {
        self.discovered_devices.insert(address.clone());
        self.schedule_endpoint_discovery(address, None);
    }

    fn schedule_endpoint_discovery(&self, address: Address, delay: Option<Duration>) {
        let zdp = self.zdp_transceiver.clone();
        let loopback = self.loopback.clone();

        spawn(async move {
            if let Some(delay) = delay {
                sleep(delay).await;
            }

            let short_id = address.short_id();
            let result = zdp.communicate(short_id, ActiveEpReq::new(short_id)).await;

            loopback
                .send(Message::ActiveEpRsp { address, result })
                .await
                .unwrap_or_else(|error| error!("Failed to send ActiveEpReq: {error:?}"));
        });
    }
}

/// Stage 2: Endpoint discovery.
impl Actor {
    fn handle_active_ep_rsp(&mut self, address: Address, result: Result<ActiveEpRsp, Error>) {
        match result {
            Ok(active_ep_rsp) => match active_ep_rsp.status() {
                Ok(Status::Success) => {
                    self.discovered_devices.remove(&address);
                    self.update_endpoints(&address, active_ep_rsp);
                    return;
                }
                Ok(error) => {
                    error!("Failed to get active endpoints for {address:?}: {error:?}");
                }
                Err(code) => {
                    warn!("Failed to get active endpoints for {address:?}: {code:#06X}");
                }
            },
            Err(error) => {
                warn!("Failed to get active endpoints for {address:?}: {error:?}");
            }
        }

        self.schedule_endpoint_discovery(address, Some(RESCHEDULE_DELAY));
    }

    fn update_endpoints(&mut self, address: &Address, active_ep_rsp: ActiveEpRsp) {
        let endpoints: BTreeSet<_> = active_ep_rsp
            .into_iter()
            .filter_map(|endpoint| {
                if let zigbee::Endpoint::Application(endpoint) = endpoint {
                    Some(endpoint)
                } else {
                    None
                }
            })
            .collect();

        self.device_endpoints
            .entry(address.clone())
            .or_default()
            .extend(endpoints.clone());

        for endpoint in &endpoints {
            self.schedule_descriptor_discovery(address.clone(), *endpoint, None);
        }
    }
}

/// Stage 3: Descriptor discovery.
impl Actor {
    fn handle_simple_desc_rsp(&mut self, address: Address, result: Result<SimpleDescRsp, Error>) {
        let Ok(simple_desc_rsp) =
            result.inspect_err(|error| error!("Failed to get simple descriptor: {error:?}"))
        else {
            return;
        };

        let Ok(Status::Success) = simple_desc_rsp.status() else {
            error!(
                "Failed to get simple descriptor: {:?}",
                simple_desc_rsp.status()
            );
            return;
        };

        let mut old_devices = self.device_endpoints.get_mut(&address);
        let device = self.discovered_endpoints.entry(address).or_default();

        for descriptor in simple_desc_rsp.into_descriptors() {
            let zigbee::Endpoint::Application(application) = descriptor.endpoint() else {
                error!("Failed to parse endpoint: {}", descriptor.endpoint());
                continue;
            };

            if let Some(old_device) = old_devices.as_mut() {
                old_device.remove(&application);
            }

            device.insert(application, descriptor.into());
        }
    }

    fn schedule_descriptor_discovery(
        &self,
        address: Address,
        endpoint: Application,
        delay: Option<Duration>,
    ) {
        let zdp = self.zdp_transceiver.clone();
        let loopback = self.loopback.clone();

        spawn(async move {
            if let Some(delay) = delay {
                sleep(delay).await;
            }

            let short_id = address.short_id();
            let result = zdp
                .communicate(short_id, SimpleDescReq::new(short_id, endpoint.into()))
                .await;

            loopback
                .send(Message::SimpleDescRsp { address, result })
                .await
                .unwrap_or_else(|error| error!("Failed to send SimpleDescRsp: {error:?}"));
        });
    }
}

/// Stage 4: Attribute discovery.
impl Actor {
    fn update_attributes(
        &mut self,
        address: Address,
        endpoint: Application,
        cluster_id: u16,
        attributes: Attributes,
    ) {
        let Some(mut device) = self.discovered_endpoints.remove(&address) else {
            return;
        };

        let Some(endpoint) = device.get_mut(&endpoint) else {
            return;
        };

        if let Some(cluster) = endpoint.input_clusters_mut().get_mut(&cluster_id) {
            cluster.set_attributes(attributes);
        }

        // TODO: Check if the device has been fully discovered.
        // If this is the case, send a message to the binding manager.
        // Else, put the device back in the queue.
    }
}
