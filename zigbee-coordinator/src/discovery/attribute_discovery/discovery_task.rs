use std::time::Duration;

use const_env::env_item;
use log::{error, trace};
use tokio::sync::mpsc::Sender;
use zcl::general::basic::readable::Id;
use zigbee::{Address, Application};

use crate::api::ReadAttributesInternal;
use crate::discovery::attribute_discovery::Message;
use crate::{RETRY, Timeout, transceiver};

#[env_item("ZIGBEE_COORDINATOR_ATTRIBUTE_DISCOVERY_TIMEOUT_SECS")]
const TIMEOUT_SECS: u64 = 5;
const TIMEOUT: Duration = Duration::from_secs(TIMEOUT_SECS);

/// A single discovery task.
#[derive(Debug)]
pub struct DiscoveryTask {
    address: Address,
    endpoint: Application,
    attributes: Box<[Id]>,
    loopback: Sender<Message>,
    zcl: Sender<transceiver::zcl::Message>,
    timeout: Duration,
}

impl DiscoveryTask {
    /// Create a new instance of `DiscoveryTask`.
    #[must_use]
    pub fn new(
        address: Address,
        endpoint: Application,
        attributes: Box<[Id]>,
        loopback: Sender<Message>,
        zcl: Sender<transceiver::zcl::Message>,
    ) -> Self {
        let factor = u32::try_from(attributes.len()).unwrap_or(u32::MAX);

        Self {
            address,
            endpoint,
            attributes,
            loopback,
            zcl,
            timeout: TIMEOUT * factor,
        }
    }

    /// Run the task.
    pub async fn run(self) {
        trace!(
            "Starting discovery of basic attributes for {}:{}.",
            self.address, self.endpoint
        );
        let mut retries = 0;

        while RETRY.retry(&mut retries).await {
            match self
                .zcl
                .read_attributes(
                    self.address.short_id(),
                    self.endpoint,
                    self.attributes.clone(),
                )
                .timeout(self.timeout)
                .await
            {
                Ok(results) => {
                    trace!(
                        "Discovered basic attributes for {}:{}. Handing over to loopback.",
                        self.address, self.endpoint
                    );

                    self.loopback
                        .send(Message::AttributesDiscovered {
                            address: self.address.clone(),
                            application: self.endpoint.clone(),
                            results,
                        })
                        .await
                        .unwrap_or_else(|error| {
                            error!("Failed to send AttributesDiscovered message: {error:?}");
                        });

                    return;
                }
                Err(error) => {
                    error!(
                        "Failed to read attributes for {}:{:#04X}: {error}",
                        self.address, self.endpoint
                    );
                }
            }
        }

        error!(
            "Failed to discover basic attributes for {}:{:#04X}.",
            self.address, self.endpoint
        );
        self.loopback
            .send(Message::DiscoveryFailed(self.address))
            .await
            .unwrap_or_else(|error| error!("Failed to send DiscoveryFailed message: {error:?}"));
    }
}
