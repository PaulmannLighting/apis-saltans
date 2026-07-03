use apis_saltans_core::Address;

use crate::{Coordinator, Error, discovery};

/// Discovery-related functions.
pub trait Discovery {
    /// Rediscover a device.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] when starting the discovery fails.
    fn rediscover(&self, address: Address) -> impl Future<Output = Result<(), Error>> + Send;
}

impl Discovery for Coordinator {
    async fn rediscover(&self, address: Address) -> Result<(), Error> {
        Ok(self
            .discovery_manager
            .send(discovery::Message::AdministrativeDiscovery(address))
            .await?)
    }
}
