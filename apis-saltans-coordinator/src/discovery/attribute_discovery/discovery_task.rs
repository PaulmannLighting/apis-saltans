use std::time::Duration;

use apis_saltans_core::{Application, FullAddress};
use apis_saltans_zcl::basic::Id;
use const_env::env_item;
use log::{debug, error, trace};
use tokio::sync::mpsc::Sender;

use crate::api::ReadAttributes;
use crate::discovery::attribute_discovery::Message;
use crate::{Timeout, transceiver};

#[env_item("ZIGBEE_COORDINATOR_ATTRIBUTE_DISCOVERY_TIMEOUT_SECS")]
const TIMEOUT_SECS: u64 = 10;
const TIMEOUT: Duration = Duration::from_secs(TIMEOUT_SECS);

const CORE_ATTRIBUTES: [Id; 6] = [
    Id::ZclVersion,
    Id::ApplicationVersion,
    Id::StackVersion,
    Id::HwVersion,
    Id::ManufacturerName,
    Id::ModelIdentifier,
];

const EXTENDED_ATTRIBUTES: [Id; 4] = [
    Id::DateCode,
    Id::PowerSource,
    Id::LocationDescription,
    Id::SwBuildId,
];

/// A single discovery task.
#[derive(Debug)]
pub struct DiscoveryTask {
    address: FullAddress,
    endpoint: Application,
    loopback: Sender<Message>,
    zcl: Sender<transceiver::zcl::Message>,
}

impl DiscoveryTask {
    /// Create a new instance of `DiscoveryTask`.
    #[must_use]
    pub const fn new(
        address: FullAddress,
        endpoint: Application,
        loopback: Sender<Message>,
        zcl: Sender<transceiver::zcl::Message>,
    ) -> Self {
        Self {
            address,
            endpoint,
            loopback,
            zcl,
        }
    }

    /// Run the task.
    pub async fn run(self) {
        trace!(
            "Starting discovery of basic attributes for {}:{}.",
            self.address, self.endpoint
        );

        let mut attributes = Vec::with_capacity(CORE_ATTRIBUTES.len() + EXTENDED_ATTRIBUTES.len());

        match self
            .zcl
            .read_attributes(self.address.ieee_address(), self.endpoint, CORE_ATTRIBUTES)
            .timeout(TIMEOUT)
            .await
        {
            Ok(core_attributes) => {
                debug!("Read core attributes");
                attributes.extend(core_attributes);
            }
            Err(error) => {
                error!(
                    "Failed to discover basic attributes for {}:{}: {error}",
                    self.address, self.endpoint
                );
                self.loopback
                    .send(Message::DiscoveryFailed(self.address))
                    .await
                    .unwrap_or_else(|error| {
                        error!("Failed to send DiscoveryFailed message: {error:?}");
                    });
                return;
            }
        }

        match self
            .zcl
            .read_attributes(
                self.address.ieee_address(),
                self.endpoint,
                EXTENDED_ATTRIBUTES,
            )
            .timeout(TIMEOUT)
            .await
        {
            Ok(extended_attributes) => {
                debug!("Read extended attributes");
                attributes.extend(extended_attributes);
            }
            Err(error) => {
                error!("Failed to read extended attributes: {error:?}");
            }
        }

        if let Err(error) = self
            .loopback
            .send(Message::AttributesDiscovered {
                address: self.address,
                application: self.endpoint,
                results: attributes.into_boxed_slice(),
            })
            .await
        {
            error!("Failed to send AttributesDiscovered message: {error:?}");
        }
    }
}
