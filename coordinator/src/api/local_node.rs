use zb_core::IeeeAddress;
use zb_hw::Ncp;

use crate::{Coordinator, Error};

/// Trait for reading local coordinator node information.
pub trait LocalNode {
    /// Return the PAN ID of the coordinator's current network.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the hardware request fails.
    fn get_pan_id(&self) -> impl Future<Output = Result<u16, Error>> + Send;

    /// Return the coordinator's IEEE address.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the hardware request fails.
    fn get_ieee_address(&self) -> impl Future<Output = Result<IeeeAddress, Error>> + Send;
}

impl LocalNode for Coordinator {
    async fn get_pan_id(&self) -> Result<u16, Error> {
        Ok(Ncp::get_pan_id(&self.ncp).await?)
    }

    async fn get_ieee_address(&self) -> Result<IeeeAddress, Error> {
        Ok(Ncp::get_ieee_address(&self.ncp).await?)
    }
}
