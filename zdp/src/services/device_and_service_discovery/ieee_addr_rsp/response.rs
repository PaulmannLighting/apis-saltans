use zb_core::{ByteSizedVec, IeeeAddress};

use crate::storage::Owned;

/// Successful IEEE Address Response payload.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(
    not(feature = "alloc"),
    allow(
        clippy::large_enum_variant,
        variant_size_differences,
        reason = "allocation-free variants own their bounded storage inline"
    )
)]
pub enum IeeeAddrRspResponse {
    /// Single-device response.
    Single {
        /// Remote device IEEE address.
        ieee_addr_remote_dev: IeeeAddress,
        /// Remote device network address.
        nwk_addr_remote_dev: u16,
    },

    /// Extended response.
    Extended {
        /// Remote device IEEE address.
        ieee_addr_remote_dev: IeeeAddress,
        /// Remote device network address.
        nwk_addr_remote_dev: u16,
        /// Starting index into the associated device list.
        start_index: u8,
        /// Associated device network addresses.
        nwk_addr_assoc_dev_list: Owned<ByteSizedVec<u16>>,
    },
}
