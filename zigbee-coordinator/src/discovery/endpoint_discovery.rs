use log::{error, warn};
use tokio::sync::mpsc::{Receiver, Sender, WeakSender};
use tokio_task_pool::Pool;
use zdp::{ActiveEpReq, Status};
use zigbee::Address;

pub use self::message::Message;
use super::descriptor_discovery;
use crate::transceiver::zdp::Handle;
use crate::{RETRY, TASK_POOL_SIZE, transceiver};

mod message;

/// Actor to discover endpoints on devices.
#[derive(Debug)]
pub struct EndpointDiscovery {
    zdp: WeakSender<transceiver::zdp::Message>,
    descriptor_discovery: Sender<descriptor_discovery::Message>,
    tasks: Pool,
}

impl EndpointDiscovery {
    /// Create a new instance of `EndpointDiscovery`.
    #[must_use]
    pub fn new(
        zdp: WeakSender<transceiver::zdp::Message>,
        descriptor_discovery: Sender<descriptor_discovery::Message>,
    ) -> Self {
        Self {
            zdp,
            descriptor_discovery,
            tasks: Pool::bounded(TASK_POOL_SIZE),
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
        self.tasks
            .spawn(discover_endpoints(
                address,
                self.zdp.clone(),
                self.descriptor_discovery.downgrade(),
            ))
            .await
            .map_or_else(
                |error| {
                    error!("Failed to spawn task: {error:?}");
                },
                drop,
            );
    }
}

/// Run the per-device endpoint discovery loop.
async fn discover_endpoints(
    address: Address,
    zdp: WeakSender<transceiver::zdp::Message>,
    descriptor_discovery: WeakSender<descriptor_discovery::Message>,
) {
    let short_id = address.short_id();
    let mut retries = 0;

    while RETRY.retry(&mut retries).await {
        let Some(zdp) = zdp.upgrade() else {
            return;
        };

        match zdp.communicate(short_id, ActiveEpReq::new(short_id)).await {
            Ok(response) => {
                if response.status() == Ok(Status::Success) {
                    let Some(descriptor_discovery) = descriptor_discovery.upgrade() else {
                        return;
                    };

                    descriptor_discovery
                        .send(descriptor_discovery::Message::Discover {
                            address: address.clone(),
                            endpoints: response.into_active_eps().into_iter().collect(),
                        })
                        .await
                        .unwrap_or_else(|error| {
                            error!("Failed to send Discover message of {address}: {error:?}");
                        });
                    return;
                }

                warn!(
                    "Got non-success status of {address}: {:?}. Retrying endpoint discovery.",
                    response.status()
                );
            }
            Err(error) => {
                warn!(
                    "Failed to discover endpoints of {address}: {error:?}. Retrying endpoint discovery."
                );
            }
        }
    }

    error!("Failed to discover endpoints of {address}.");
}
