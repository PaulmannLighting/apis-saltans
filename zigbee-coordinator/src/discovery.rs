use log::{error, info};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender, WeakSender, channel};

use self::attribute_discovery::AttributeDiscovery;
use self::descriptor_discovery::DescriptorDiscovery;
use self::endpoint_discovery::EndpointDiscovery;
pub use self::message::Message;
use crate::{MPSC_CHANNEL_SIZE, binding, transceiver};

mod attribute_discovery;
mod descriptor_discovery;
mod endpoint_discovery;
mod message;

/// The device discovery actor.
#[derive(Debug)]
pub struct Actor {
    endpoint_discovery: Sender<endpoint_discovery::Message>,
}

impl Actor {
    /// Create a new discovery actor.
    #[must_use]
    pub fn new(
        zcl: WeakSender<transceiver::zcl::Message>,
        zdp: WeakSender<transceiver::zdp::Message>,
        binding_manager: Sender<binding::Message>,
    ) -> Self {
        let (attribute_discovery, ad_tx) = AttributeDiscovery::new(zcl, binding_manager);
        spawn(attribute_discovery.run());
        let (descriptor_discovery, dd_tx) = DescriptorDiscovery::new(zdp.clone(), ad_tx);
        spawn(descriptor_discovery.run());
        let (ed_tx, ed_rx) = channel(MPSC_CHANNEL_SIZE);
        let endpoint_discovery = EndpointDiscovery::new(zdp, dd_tx);
        spawn(endpoint_discovery.run(ed_rx));

        Self {
            endpoint_discovery: ed_tx,
        }
    }

    /// Run the discovery actor.
    pub async fn run(self, mut messages: Receiver<Message>) {
        while let Some(event) = messages.recv().await {
            let address = match event {
                Message::DeviceJoined(address) => {
                    info!("Device joined: {address}");
                    address
                }
                Message::DeviceRejoined { address, secured } => {
                    info!("Device rejoined: {address}, secured: {secured}");
                    address
                }
                Message::DeviceAnnounced {
                    address,
                    capabilities,
                } => {
                    info!("Device announced: {address}, capabilities: {capabilities}");
                    address
                }
            };

            self.endpoint_discovery
                .send(endpoint_discovery::Message::Discover(address))
                .await
                .unwrap_or_else(|error| error!("Failed to send discover message: {error:?}"));
        }
    }
}
