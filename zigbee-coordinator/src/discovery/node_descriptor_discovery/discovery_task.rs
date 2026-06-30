use std::time::Duration;

use const_env::env_item;
use log::{error, trace, warn};
use tokio::sync::mpsc::Sender;
use zdp::{NodeDescReq, Status};
use zigbee::types::tlv::FragmentationParameters;
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
    loopback: Sender<Message>,
    zdp: Sender<transceiver::zdp::Message>,
}

impl DiscoveryTask {
    /// Create a new instance of `DiscoveryTask`.
    #[must_use]
    pub fn new(
        address: Address,
        loopback: Sender<Message>,
        zdp: Sender<transceiver::zdp::Message>,
    ) -> Self {
        Self {
            address,
            loopback,
            zdp,
        }
    }

    /// Run the task.
    pub async fn run(self) {
        trace!("Starting discovery of descriptor for {}", self.address);
        let short_id = self.address.short_id();
        let mut retries = 0;

        while RETRY.retry(&mut retries).await {
            match self
                .zdp
                .communicate(
                    short_id,
                    NodeDescReq::from(FragmentationParameters::new(short_id, None, None)),
                )
                .timeout(TIMEOUT)
                .await
            {
                Ok(response) => match response.try_into() {
                    Ok(descriptor) => {
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
                    Err(Ok(status)) => {
                        warn!("Failed to get descriptor for {}: {status:?}", self.address);
                    }
                    Err(Err(status)) => {
                        warn!(
                            "Failed to get descriptor for {}: {status:#04X}",
                            self.address
                        );
                    }
                },
                Err(error) => {
                    warn!("Failed to get descriptor for {}: {error:?}", self.address);
                }
            }
        }

        error!("Failed to get descriptor for {}.", self.address);
        self.loopback
            .send(Message::DiscoveryFailed(self.address))
            .await
            .unwrap_or_else(|error| {
                error!("Failed to send DiscoveryFailed message: {error:?}");
            });
    }
}
