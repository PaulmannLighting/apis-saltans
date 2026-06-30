use std::time::Duration;

use const_env::env_item;
use log::{error, trace};
use tokio::sync::mpsc::Sender;
use zcl::general::basic::readable::Id;
use zigbee::{Address, Application};

use crate::api::ReadAttributesInternal;
use crate::discovery::attribute_discovery::{ATTRIBUTES, Message};
use crate::{RETRY, Timeout, transceiver};

#[env_item("ZIGBEE_COORDINATOR_ATTRIBUTE_DISCOVERY_TIMEOUT_SECS")]
const TIMEOUT_SECS: u64 = 5;
const TIMEOUT: Duration = Duration::from_secs(TIMEOUT_SECS);

/// A single discovery task.
#[derive(Debug)]
pub struct DiscoveryTask {
    address: Address,
    application: Application,
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
        application: Application,
        attributes: Box<[Id]>,
        loopback: Sender<Message>,
        zcl: Sender<transceiver::zcl::Message>,
    ) -> Self {
        let factor = u32::try_from(attributes.len()).unwrap_or(u32::MAX);

        Self {
            address,
            application,
            attributes,
            loopback,
            zcl,
            timeout: TIMEOUT * factor,
        }
    }

    /// Run the task.
    pub async fn run(self) {
        trace!("Starting discovery of basic attributes for {address}:{application}.");
        let mut retries = 0;

        while RETRY.retry(&mut retries).await {
            match self
                .zcl
                .read_attributes_one_by_one(
                    self.address.short_id(),
                    self.application,
                    ATTRIBUTES.into(),
                )
                .timeout(self.timeout)
                .await
            {
                Ok(results) => {
                    trace!(
                        "Discovered basic attributes for {address}:{application}. Handing over to loopback."
                    );
                    self.loopback
                        .send(Message::AttributesDiscovered {
                            address: self.address.clone(),
                            application,
                            results,
                        })
                        .await
                        .unwrap_or_else(|error| {
                            error!("Failed to send AttributesDiscovered message: {error:?}");
                        });
                    return;
                }
                Err(error) => {
                    error!("Failed to read attributes for {address}:{application:#04X}: {error}");
                }
            }
        }

        error!("Failed to discover basic attributes for {address}:{application:#04X}.");
        self.loopback
            .send(Message::DiscoveryFailed { address })
            .await
            .unwrap_or_else(|error| error!("Failed to send DiscoveryFailed message: {error:?}"));
    }
}
