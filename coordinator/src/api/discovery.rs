use apis_saltans_core::IeeeAddress;
use either::Either;

use crate::{Coordinator, Error, NetworkManager, discovery};

/// Discovery-related functions.
pub trait Discovery {
    /// Rediscover a device.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] when starting the discovery fails.
    fn rediscover(
        &self,
        ieee_address: IeeeAddress,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl Discovery for Coordinator {
    async fn rediscover(&self, ieee_address: IeeeAddress) -> Result<(), Error> {
        let Some(address) = self.get_full_address(Either::Left(ieee_address)).await? else {
            return Err(Error::UnknownDevice(ieee_address));
        };
        Ok(self
            .discovery_manager
            .send(discovery::Message::AdministrativeDiscovery(address))
            .await?)
    }
}
