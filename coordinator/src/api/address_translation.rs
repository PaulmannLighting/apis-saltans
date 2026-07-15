use zb_core::IeeeAddress;
use zb_hw::Ncp;

use crate::{Coordinator, Error};

/// Trait for resolving between NWK short addresses and IEEE addresses.
///
/// These methods delegate to the hardware/NCP address translation table. Applications that keep
/// their own device registry may use this as a fallback or to refresh address mappings after
/// rejoins.
pub trait AddressTranslation {
    /// Resolve a NWK short address to an IEEE address.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the NCP cannot resolve the address or the hardware request fails.
    fn short_id_to_ieee_address(
        &self,
        short_id: u16,
    ) -> impl Future<Output = Result<IeeeAddress, Error>> + Send;

    /// Resolve an IEEE address to a NWK short address.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the NCP cannot resolve the address or the hardware request fails.
    fn ieee_address_to_short_id(
        &self,
        ieee_address: IeeeAddress,
    ) -> impl Future<Output = Result<u16, Error>> + Send;
}

impl AddressTranslation for Coordinator {
    async fn short_id_to_ieee_address(&self, short_id: u16) -> Result<IeeeAddress, Error> {
        Ok(Ncp::short_id_to_ieee_address(&self.ncp, short_id).await?)
    }

    async fn ieee_address_to_short_id(&self, ieee_address: IeeeAddress) -> Result<u16, Error> {
        Ok(Ncp::ieee_address_to_short_id(&self.ncp, ieee_address).await?)
    }
}
