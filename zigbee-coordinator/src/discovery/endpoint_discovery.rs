use std::time::Duration;

use log::{error, warn};
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::sleep;
use zdp::{ActiveEpReq, Status};
use zigbee::Address;

pub use self::message::Message;
use super::descriptor_discovery;
use crate::transceiver;
use crate::transceiver::zdp::Handle;

mod message;

/// Actor to discover endpoints on devices.
#[derive(Debug)]
pub struct EndpointDiscovery {
    zdp: Sender<transceiver::zdp::Message>,
    descriptor_discovery: Sender<descriptor_discovery::Message>,
    max_retries: usize,
    retry_delay: Duration,
}

impl EndpointDiscovery {
    /// Create a new instance of `EndpointDiscovery`.
    #[must_use]
    pub fn new(
        zdp: Sender<transceiver::zdp::Message>,
        descriptor_discovery: Sender<descriptor_discovery::Message>,
        max_retries: usize,
        retry_delay: Duration,
    ) -> Self {
        Self {
            zdp,
            descriptor_discovery,
            max_retries,
            retry_delay,
        }
    }

    /// Run the actor.
    pub async fn run(self, mut messages: Receiver<Message>) {
        while let Some(message) = messages.recv().await {
            match message {
                Message::Discover(address) => {
                    self.discover_endpoints(address).await;
                }
            }
        }
    }

    /// Discover endpoints on the given device in a separate task.
    async fn discover_endpoints(&self, address: Address) {
        spawn(discover_endpoints(
            address,
            self.zdp.clone(),
            self.descriptor_discovery.clone(),
            self.max_retries,
            self.retry_delay,
        ));
    }
}

/// Run the per-device endpoint discovery loop.
async fn discover_endpoints(
    address: Address,
    zdp: Sender<transceiver::zdp::Message>,
    descriptor_discovery: Sender<descriptor_discovery::Message>,
    max_retries: usize,
    retry_delay: Duration,
) {
    let short_id = address.short_id();
    let mut retries = 0;

    loop {
        if retries > max_retries {
            error!(
                "Failed to discover endpoints of {address} after {max_retries} retries. Giving up."
            );
            return;
        }

        if retries > 0 {
            sleep(retry_delay).await;
        }

        retries += 1;

        match zdp.communicate(short_id, ActiveEpReq::new(short_id)).await {
            Ok(response) => {
                if let Ok(Status::Success) = response.status() {
                    descriptor_discovery
                        .send(descriptor_discovery::Message::Discover {
                            address,
                            endpoints: response.into_active_eps().into_iter().collect(),
                        })
                        .await
                        .unwrap_or_else(|error| {
                            error!("Failed to send Discover message: {error:?}")
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
}
