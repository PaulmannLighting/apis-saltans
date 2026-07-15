use zb_hw::Ncp;

use crate::{Coordinator, Error};

/// Trait for requesting route discovery through the hardware/NCP.
pub trait Routing {
    /// Broadcast a route request with the given radius.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the hardware request fails.
    fn route_request(&self, radius: u8) -> impl Future<Output = Result<(), Error>> + Send;
}

impl Routing for Coordinator {
    async fn route_request(&self, radius: u8) -> Result<(), Error> {
        Ok(Ncp::route_request(&self.ncp, radius).await?)
    }
}
