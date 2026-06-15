use log::{error, warn};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender};
use zdp::{ActiveEpReq, Status};
use zigbee::Address;

pub use self::message::Message;
use super::descriptor_discovery;
use crate::transceiver::zdp::Handle;
use crate::{RETRY, transceiver};

mod message;

/// Actor to discover endpoints on devices.
#[derive(Debug)]
pub struct EndpointDiscovery {
    zdp: Sender<transceiver::zdp::Message>,
    descriptor_discovery: Sender<descriptor_discovery::Message>,
}

impl EndpointDiscovery {
    /// Create a new instance of `EndpointDiscovery`.
    #[must_use]
    pub const fn new(
        zdp: Sender<transceiver::zdp::Message>,
        descriptor_discovery: Sender<descriptor_discovery::Message>,
    ) -> Self {
        Self {
            zdp,
            descriptor_discovery,
        }
    }

    /// Run the actor.
    pub async fn run(self, mut messages: Receiver<Message>) {
        while let Some(message) = messages.recv().await {
            match message {
                Message::Discover(address) => {
                    self.discover_endpoints(address);
                }
            }
        }
    }

    /// Discover endpoints on the given device in a separate task.
    fn discover_endpoints(&self, address: Address) {
        spawn(discover_endpoints(
            address,
            self.zdp.clone(),
            self.descriptor_discovery.clone(),
        ));
    }
}

/// Run the per-device endpoint discovery loop.
async fn discover_endpoints(
    address: Address,
    zdp: Sender<transceiver::zdp::Message>,
    descriptor_discovery: Sender<descriptor_discovery::Message>,
) {
    let short_id = address.short_id();
    let mut retries = 0;

    while RETRY.retry(&mut retries).await {
        match zdp.communicate(short_id, ActiveEpReq::new(short_id)).await {
            Ok(response) => {
                if response.status() == Ok(Status::Success) {
                    descriptor_discovery
                        .send(descriptor_discovery::Message::Discover {
                            address,
                            endpoints: response.into_active_eps().into_iter().collect(),
                        })
                        .await
                        .unwrap_or_else(|error| {
                            error!("Failed to send Discover message: {error:?}");
                        });
                    return;
                }

                warn!(
                    "Got non-success status: {:?}. Retrying endpoint discovery.",
                    response.status()
                );
            }
            Err(error) => {
                warn!("Failed to discover endpoints: {error:?}. Retrying endpoint discovery.");
            }
        }
    }

    error!("Failed to discover endpoints of {address}.");
}
