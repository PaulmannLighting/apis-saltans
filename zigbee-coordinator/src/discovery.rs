use std::collections::BTreeMap;
use std::time::Duration;

use log::{error, trace, warn};
use macaddr::MacAddr8;
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use tokio::time::sleep;
use zdp::{ActiveEpReq, ActiveEpRsp, SimpleDescRsp, Status};
use zigbee::{Address, Application};
use zigbee_hw::Event;

pub use self::message::Message;
use crate::device::{Device, Endpoint};
use crate::transceiver::zdp::Handle;
use crate::{Error, binding, transceiver};

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
    devices: BTreeMap<MacAddr8, Device>,
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
            devices: BTreeMap::new(),
        };

        (instance, tx)
    }

    /// Run the discovery actor.
    pub async fn run(mut self) {
        while let Some(message) = self.inbox.recv().await {
            match message {
                Message::Event(event) => self.handle_event(event),
                Message::ActiveEpRsp { address, result } => {
                    self.handle_active_ep_rsp(address, result)
                }
                Message::SimpleDescRsp {
                    address,
                    endpoint,
                    result,
                } => self.handle_simple_desc_rsp(address, endpoint, result),
            }
        }
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::DeviceJoined(address) => self.handle_join(address),
            Event::DeviceLeft(address) => self.handle_leave(address),
            _ => trace!("Unhandled event: {event:?}"),
        }
    }

    fn handle_active_ep_rsp(&mut self, address: Address, result: Result<ActiveEpRsp, Error>) {
        match result {
            Ok(active_ep_rsp) => match active_ep_rsp.status() {
                Ok(Status::Success) => {
                    self.update_endpoints(address, active_ep_rsp);
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

    fn handle_simple_desc_rsp(
        &mut self,
        address: Address,
        endpoint: Application,
        result: Result<SimpleDescRsp, Error>,
    ) {
        todo!("Handle simple descriptor response.");
    }

    fn update_endpoints(&mut self, address: Address, active_ep_rsp: ActiveEpRsp) {
        let Some(device) = self.devices.get_mut(&address.ieee_address()) else {
            return;
        };

        let endpoints: Box<[Application]> = active_ep_rsp
            .into_iter()
            .filter_map(|endpoint| {
                if let zigbee::Endpoint::Application(endpoint) = endpoint {
                    Some(endpoint)
                } else {
                    None
                }
            })
            .collect();

        for endpoint in &endpoints {
            device
                .endpoints_mut()
                .insert(*endpoint, Endpoint::new(*endpoint, BTreeMap::default()));
        }

        for endpoint in endpoints {
            self.schedule_descriptor_discovery(address.clone(), endpoint);
        }
    }

    fn handle_join(&mut self, address: Address) {
        self.devices
            .insert(address.ieee_address(), Device::from(address.clone()));
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

    fn schedule_descriptor_discovery(&self, address: Address, endpoint: Application) {
        let zdp = self.zdp_transceiver.clone();
        let loopback = self.loopback.clone();
        todo!("Schedule descriptor discovery for {address:?}, {endpoint:?}.");
    }

    fn handle_leave(&mut self, address: Address) {
        if let Some(device) = self.devices.remove(&address.ieee_address()) {
            todo!("Notify network manager.")
        }
    }
}
