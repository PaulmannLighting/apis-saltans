use std::collections::BTreeMap;

use zb_core::{Application, IeeeAddress};
pub use zb_hw::Clusters;
use zb_hw::Ncp;

use crate::{Coordinator, Error};

/// Trait for reading local coordinator node information.
pub trait LocalNode {
    /// Return the local application endpoint cluster sets advertised by the coordinator.
    ///
    /// The returned map is keyed by application endpoint ID. Each [`Clusters`] value contains the
    /// input and output clusters configured for that local endpoint.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the hardware request fails.
    fn get_endpoints(
        &self,
    ) -> impl Future<Output = Result<BTreeMap<Application, Clusters>, Error>> + Send;

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
    async fn get_endpoints(&self) -> Result<BTreeMap<Application, Clusters>, Error> {
        Ok(Ncp::get_endpoints(&self.ncp).await?)
    }

    async fn get_pan_id(&self) -> Result<u16, Error> {
        Ok(Ncp::get_pan_id(&self.ncp).await?)
    }

    async fn get_ieee_address(&self) -> Result<IeeeAddress, Error> {
        Ok(Ncp::get_ieee_address(&self.ncp).await?)
    }
}
