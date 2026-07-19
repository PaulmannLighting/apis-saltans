use zb_core::IeeeAddress;
pub use zb_hw::Clusters;
use zb_hw::Ncp;
use zb_zdp::SimpleDescriptor;

use crate::{Coordinator, Error};

/// Trait for reading local coordinator node information.
pub trait LocalNode {
    /// Return the local application endpoints advertised by the NCP.
    ///
    /// Each [`SimpleDescriptor`] contains the endpoint ID, profile, device ID, application version,
    /// and input and output cluster lists used for local ZDP and binding operations.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the hardware request fails.
    fn get_endpoints(&self) -> impl Future<Output = Result<Box<[SimpleDescriptor]>, Error>> + Send;

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
    async fn get_endpoints(&self) -> Result<Box<[SimpleDescriptor]>, Error> {
        Ok(Ncp::get_endpoints(&self.ncp).await?)
    }

    async fn get_pan_id(&self) -> Result<u16, Error> {
        Ok(Ncp::get_pan_id(&self.ncp).await?)
    }

    async fn get_ieee_address(&self) -> Result<IeeeAddress, Error> {
        Ok(Ncp::get_ieee_address(&self.ncp).await?)
    }
}
