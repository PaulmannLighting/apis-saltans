use apis_saltans_hw::{Ncp, WeakNcpHandle};
use log::{error, info, trace};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender, WeakSender};

use self::attribute_discovery::AttributeDiscovery;
use self::endpoint_descriptor_discovery::EndpointDescriptorDiscovery;
use self::endpoint_discovery::EndpointDiscovery;
pub use self::message::Message;
use crate::discovery::node_descriptor_discovery::NodeDescriptorDiscovery;
use crate::{binding, transceiver};

mod attribute_discovery;
mod endpoint_descriptor_discovery;
mod endpoint_discovery;
mod message;
mod node_descriptor_discovery;

/// The device discovery actor.
#[derive(Debug)]
pub struct Actor {
    node_descriptor_discovery: Sender<node_descriptor_discovery::Message>,
    ncp: WeakNcpHandle,
}

impl Actor {
    /// Create a new discovery actor.
    #[must_use]
    pub fn new(
        zcl: WeakSender<transceiver::zcl::Message>,
        zdp: WeakSender<transceiver::zdp::Message>,
        binding_manager: Sender<binding::Message>,
        ncp: WeakNcpHandle,
    ) -> Self {
        let (attribute_discovery, ad_tx) = AttributeDiscovery::new(zcl, binding_manager);
        spawn(attribute_discovery.run());
        let (endpoint_descriptor_discovery, edd_tx) =
            EndpointDescriptorDiscovery::new(zdp.clone(), ad_tx);
        spawn(endpoint_descriptor_discovery.run());
        let (endpoint_discovery, ed_tx) = EndpointDiscovery::new(zdp.clone(), edd_tx);
        spawn(endpoint_discovery.run());
        let (node_descriptor_discovery, ndd_tx) = NodeDescriptorDiscovery::new(zdp, ed_tx);
        spawn(node_descriptor_discovery.run());

        Self {
            node_descriptor_discovery: ndd_tx,
            ncp,
        }
    }

    /// Start the discovery manager.
    pub fn spawn(
        discovery_rx: Receiver<Message>,
        zcl_tx: WeakSender<transceiver::zcl::Message>,
        zdp_tx: WeakSender<transceiver::zdp::Message>,
        binding_tx: Sender<binding::Message>,
        ncp: WeakNcpHandle,
    ) {
        let discovery_manager = Self::new(zcl_tx, zdp_tx, binding_tx, ncp);
        spawn(discovery_manager.run(discovery_rx));
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

            trace!("Start descriptor discovery for {address}");

            if let Some(ncp) = self.ncp.upgrade() {
                ncp.route_request(64).await.unwrap_or_else(|error| {
                    error!("Failed to issue route request: {error}");
                });
            }

            self.node_descriptor_discovery
                .send(node_descriptor_discovery::Message::Discover(address))
                .await
                .unwrap_or_else(|error| error!("Failed to send discover message: {error:?}"));
        }
    }
}
