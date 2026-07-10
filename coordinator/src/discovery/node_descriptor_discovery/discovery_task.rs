use std::time::Duration;

use const_env::env_item;
use log::{error, info, trace, warn};
use tokio::sync::mpsc::Sender;
use zb_core::short_id::Device;
use zb_core::types::tlv::FragmentationParameters;
use zb_core::{FullAddress, IeeeAddress};
use zb_zdp::NodeDescReq;

use super::Message;
use crate::transceiver::zdp::Handle;
use crate::{Timeout, transceiver};

#[env_item("ZIGBEE_COORDINATOR_DESCRIPTOR_DISCOVERY_TIMEOUT_SECS")]
const TIMEOUT_SECS: u64 = 5;
const TIMEOUT: Duration = Duration::from_secs(TIMEOUT_SECS);

/// Task to discover a descriptor on a device.
#[derive(Debug)]
pub struct DiscoveryTask {
    address: FullAddress,
    loopback: Sender<Message>,
    zdp: Sender<transceiver::zdp::Message>,
}

impl DiscoveryTask {
    /// Create a new instance of `DiscoveryTask`.
    #[must_use]
    pub const fn new(
        address: FullAddress,
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

        match self
            .zdp
            .communicate(
                short_id,
                NodeDescReq::from(FragmentationParameters::new(short_id.into(), None, None)),
            )
            .timeout(TIMEOUT)
            .await
        {
            Ok(response) => match response.try_into() {
                Ok(descriptor) => {
                    info!("Descriptor discovered for {}: {descriptor:?}", self.address);
                    self.loopback
                        .send(Message::DescriptorDiscovered {
                            address: self.address,
                            descriptor: Box::new(descriptor),
                        })
                        .await
                        .unwrap_or_else(|error| {
                            error!("Failed to send DescriptorsDiscovered message: {error:?}");
                        });
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
                error!("Failed to get descriptor for {}: {error}", self.address);
                self.loopback
                    .send(Message::DiscoveryFailed(self.address))
                    .await
                    .unwrap_or_else(|error| {
                        error!("Failed to send DiscoveryFailed message: {error:?}");
                    });
            }
        }
    }
}
