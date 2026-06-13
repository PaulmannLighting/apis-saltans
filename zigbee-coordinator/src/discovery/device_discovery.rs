use std::collections::BTreeMap;
use std::time::Duration;

use log::warn;
use tokio::sync::mpsc::Sender;
use tokio::time::sleep;
use zdp::{ActiveEpReq, SimpleDescReq, SimpleDescriptor};
use zigbee::{Address, Endpoint};

use crate::transceiver;
use crate::transceiver::zdp::Handle;

const RETRY_DELAY: Duration = Duration::from_secs(30);

struct DiscoveredDevice {}

// TODO: Work in progress to implement per-device discovery futures.
pub struct DeviceDiscovery {
    address: Address,
    zcl: Sender<transceiver::zcl::Message>,
    zdp: Sender<transceiver::zdp::Message>,
}

impl DeviceDiscovery {
    pub async fn run(self) -> DiscoveredDevice {
        let short_id = self.address.short_id();

        let mut descriptors = BTreeMap::new();

        for endpoint in self.retrieve_endpoints(short_id).await {
            for descriptor in self.retrieve_descriptors(short_id, endpoint).await {
                descriptors.insert(descriptor.endpoint(), descriptor);
            }
        }

        // TODO: Build device object.
        DiscoveredDevice {}
    }

    async fn retrieve_endpoints(&self, short_id: u16) -> Box<[Endpoint]> {
        loop {
            if let Ok(response) = self
                .zdp
                .communicate(short_id, ActiveEpReq::new(short_id))
                .await
                && response.status() == Ok(zdp::Status::Success)
            {
                return response.into_active_eps();
            }

            warn!(
                "Failed to get active endpoints for {short_id:06X}. Retrying in {} seconds.",
                RETRY_DELAY.as_secs()
            );
            sleep(RETRY_DELAY).await;
        }
    }

    async fn retrieve_descriptors(
        &self,
        short_id: u16,
        endpoint: Endpoint,
    ) -> Box<[SimpleDescriptor]> {
        loop {
            if let Ok(response) = self
                .zdp
                .communicate(short_id, SimpleDescReq::new(short_id, endpoint.into()))
                .await
                && response.status() == Ok(zdp::Status::Success)
            {
                return response.into_descriptors();
            }

            warn!(
                "Failed to get endpoint descriptor for {short_id:06X}:{endpoint}. Retrying in {} seconds.",
                RETRY_DELAY.as_secs()
            );
            sleep(RETRY_DELAY).await;
        }
    }
}
