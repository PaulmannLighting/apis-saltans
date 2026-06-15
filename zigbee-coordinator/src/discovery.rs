use std::time::Duration;

use log::{error, info, trace};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use zigbee_hw::Event;

pub use self::attribute_discovery::EndpointInfo;
use crate::discovery::attribute_discovery::AttributeDiscovery;
use crate::discovery::descriptor_discovery::DescriptorDiscovery;
use crate::discovery::endpoint_discovery::EndpointDiscovery;
use crate::{binding, transceiver};

mod attribute_discovery;
mod descriptor_discovery;
mod device_discovery;
mod endpoint;
mod endpoint_discovery;
mod message;

const MAX_RETRIES: usize = 120;
const RETRY_DELAY: Duration = Duration::from_secs(30);

/// The device discovery actor.
#[derive(Debug)]
pub struct Actor {
    ed_tx: Sender<endpoint_discovery::Message>,
    ed_rx: Receiver<endpoint_discovery::Message>,
    endpoint_discovery: EndpointDiscovery,
    descriptor_discovery: DescriptorDiscovery,
    attribute_discovery: AttributeDiscovery,
}

impl Actor {
    /// Create a new discovery actor.
    #[must_use]
    pub fn new(
        channel_size: usize,
        zcl: Sender<transceiver::zcl::Message>,
        zdp: Sender<transceiver::zdp::Message>,
        binding_manager: Sender<binding::Message>,
    ) -> Self {
        let (attribute_discovery, ad_tx) =
            AttributeDiscovery::new(channel_size, zcl, binding_manager, MAX_RETRIES, RETRY_DELAY);
        let (descriptor_discovery, dd_tx) =
            DescriptorDiscovery::new(channel_size, zdp.clone(), ad_tx, MAX_RETRIES, RETRY_DELAY);
        let endpoint_discovery = EndpointDiscovery::new(zdp, dd_tx, MAX_RETRIES, RETRY_DELAY);
        let (ed_tx, ed_rx) = channel(channel_size);

        Self {
            ed_tx,
            ed_rx,
            endpoint_discovery,
            descriptor_discovery,
            attribute_discovery,
        }
    }

    /// Run the discovery actor.
    pub async fn run(self, mut messages: Receiver<Event>) {
        spawn(self.attribute_discovery.run());
        spawn(self.descriptor_discovery.run());
        spawn(self.endpoint_discovery.run(self.ed_rx));

        while let Some(event) = messages.recv().await {
            let address = match event {
                Event::DeviceJoined(address) => {
                    info!("Device joined: {address:?}");
                    address
                }
                Event::DeviceRejoined { address, secured } => {
                    info!("Device rejoined: {address:?}, secured: {secured}");
                    address
                }
                _ => {
                    trace!("Unhandled event: {event:?}");
                    continue;
                }
            };

            self.ed_tx
                .send(endpoint_discovery::Message::Discover(address))
                .await
                .unwrap_or_else(|error| error!("Failed to send discover message: {error:?}"))
        }
    }
}
