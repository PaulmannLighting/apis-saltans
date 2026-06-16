use log::{error, info};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender, WeakSender, channel};

pub use self::attribute_discovery::EndpointInfo;
pub use self::message::Message;
use crate::discovery::attribute_discovery::AttributeDiscovery;
use crate::discovery::descriptor_discovery::DescriptorDiscovery;
use crate::discovery::endpoint_discovery::EndpointDiscovery;
use crate::{MPSC_CHANNEL_SIZE, binding, transceiver};

mod attribute_discovery;
mod descriptor_discovery;
mod endpoint_discovery;
mod message;

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
        zcl: WeakSender<transceiver::zcl::Message>,
        zdp: WeakSender<transceiver::zdp::Message>,
        binding_manager: WeakSender<binding::Message>,
    ) -> Self {
        let (attribute_discovery, ad_tx) = AttributeDiscovery::new(zcl, binding_manager);
        let (descriptor_discovery, dd_tx) = DescriptorDiscovery::new(zdp.clone(), ad_tx);
        let endpoint_discovery = EndpointDiscovery::new(zdp, dd_tx);
        let (ed_tx, ed_rx) = channel(MPSC_CHANNEL_SIZE);

        Self {
            ed_tx,
            ed_rx,
            endpoint_discovery,
            descriptor_discovery,
            attribute_discovery,
        }
    }

    /// Run the discovery actor.
    pub async fn run(self, mut messages: Receiver<Message>) {
        spawn(self.attribute_discovery.run());
        spawn(self.descriptor_discovery.run());
        spawn(self.endpoint_discovery.run(self.ed_rx));

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
            };

            self.ed_tx
                .send(endpoint_discovery::Message::Discover(address))
                .await
                .unwrap_or_else(|error| error!("Failed to send discover message: {error:?}"));
        }
    }
}
