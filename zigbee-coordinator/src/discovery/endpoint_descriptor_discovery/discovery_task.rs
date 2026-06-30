use std::time::Duration;

use const_env::env_item;
use log::{error, trace, warn};
use tokio::sync::mpsc::Sender;
use zdp::{SimpleDescReq, Status};
use zigbee::{Address, Endpoint};

use super::Message;
use crate::transceiver::zdp::Handle;
use crate::{RETRY, Timeout, transceiver};

#[env_item("ZIGBEE_COORDINATOR_DESCRIPTOR_DISCOVERY_TIMEOUT_SECS")]
const TIMEOUT_SECS: u64 = 5;
const TIMEOUT: Duration = Duration::from_secs(TIMEOUT_SECS);

/// Task to discover a descriptor on a device.
#[derive(Debug)]
pub struct DiscoveryTask {
    address: Address,
    endpoint: Endpoint,
    loopback: Sender<Message>,
    zdp: Sender<transceiver::zdp::Message>,
}

impl DiscoveryTask {
    /// Create a new instance of `DiscoveryTask`.
    #[must_use]
    pub const fn new(
        address: Address,
        endpoint: Endpoint,
        loopback: Sender<Message>,
        zdp: Sender<transceiver::zdp::Message>,
    ) -> Self {
        Self {
            address,
            endpoint,
            loopback,
            zdp,
        }
    }

    /// Run the task.
    pub async fn run(self) {
        trace!(
            "Starting discovery of descriptor for {}:{}.",
            self.address, self.endpoint
        );
        let short_id = self.address.short_id();
        let mut retries = 0;

        while RETRY.retry(&mut retries).await {
            match self
                .zdp
                .communicate(short_id, SimpleDescReq::new(short_id, self.endpoint))
                .timeout(TIMEOUT)
                .await
            {
                Ok(response) => {
                    if response.status() == Ok(Status::Success) {
                        trace!("Got descriptor for {}:{}.", self.address, self.endpoint);

                        let Some(descriptor) = response.into_descriptor() else {
                            error!(
                                "Got descriptor for {}:{} but it was invalid.",
                                self.address, self.endpoint
                            );
                            continue;
                        };

                        trace!(
                            "Sending descriptor for {}:{} to loopback.",
                            self.address, self.endpoint
                        );
                        self.loopback
                            .send(Message::DescriptorDiscovered {
                                address: self.address.clone(),
                                descriptor: Box::new(descriptor),
                            })
                            .await
                            .unwrap_or_else(|error| {
                                error!("Failed to send DescriptorsDiscovered message: {error:?}");
                            });

                        return;
                    }

                    warn!(
                        "Failed to get descriptor for {}:{}: {:?}",
                        self.address,
                        self.endpoint,
                        response.status()
                    );
                }
                Err(error) => {
                    warn!(
                        "Failed to get descriptor for {}:{}: {error:?}",
                        self.address, self.endpoint
                    );
                }
            }
        }

        error!(
            "Failed to get descriptor for {}:{}.",
            self.address, self.endpoint
        );
        self.loopback
            .send(Message::DiscoveryFailed(self.address))
            .await
            .unwrap_or_else(|error| {
                error!("Failed to send DiscoveryFailed message: {error:?}");
            });
    }
}
