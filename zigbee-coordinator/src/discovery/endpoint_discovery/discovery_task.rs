use std::time::Duration;

use const_env::env_item;
use log::{error, trace, warn};
use tokio::sync::mpsc::Sender;
use zdp::{ActiveEpReq, Status};
use zigbee::Address;

use super::Message;
use crate::transceiver::zdp::Handle;
use crate::{RETRY, Timeout, transceiver};

#[env_item("ZIGBEE_COORDINATOR_ENDPOINT_DISCOVERY_TIMEOUT_SECS")]
const TIMEOUT_SECS: u64 = 5;
const TIMEOUT: Duration = Duration::from_secs(TIMEOUT_SECS);

/// A single discovery task.
#[derive(Debug)]
pub struct DiscoveryTask {
    address: Address,
    zdp: Sender<transceiver::zdp::Message>,
    loopback: Sender<Message>,
}

impl DiscoveryTask {
    /// Create a new instance of `DiscoveryTask`.
    #[must_use]
    pub fn new(
        address: Address,
        zdp: Sender<transceiver::zdp::Message>,
        loopback: Sender<Message>,
    ) -> Self {
        Self {
            address,
            zdp,
            loopback,
        }
    }

    /// Run the task.
    pub async fn run(self) {
        trace!("Starting endpoint discovery of {}.", self.address);

        let short_id = self.address.short_id();
        let mut retries = 0;

        while RETRY.retry(&mut retries).await {
            match self
                .zdp
                .communicate(short_id, ActiveEpReq::new(short_id))
                .timeout(TIMEOUT)
                .await
            {
                Ok(response) => {
                    if response.status() == Ok(Status::Success) {
                        trace!(
                            "Discovered endpoints of {}. Handing over to descriptor discovery.",
                            self.address
                        );

                        self.loopback
                            .send(Message::Discovered {
                                address: self.address.clone(),
                                endpoints: response.into_active_eps().into_iter().collect(),
                            })
                            .await
                            .unwrap_or_else(|error| {
                                error!(
                                    "Failed to send Discovered message of {}: {error:?}",
                                    self.address
                                );
                            });

                        return;
                    }

                    warn!(
                        "Got non-success status of {}: {:?}. Retrying endpoint discovery.",
                        self.address,
                        response.status()
                    );
                }
                Err(error) => {
                    warn!(
                        "ZDP communication failed while discovering endpoints of {}: {error:?}. Retrying endpoint discovery.",
                        self.address
                    );
                }
            }
        }

        error!("Failed to discover endpoints of {}.", self.address);
        self.loopback
            .send(Message::DiscoveryFailed(self.address))
            .await
            .unwrap_or_else(|error| error!("Failed to send DiscoveryFailed message: {error:?}"));
    }
}
