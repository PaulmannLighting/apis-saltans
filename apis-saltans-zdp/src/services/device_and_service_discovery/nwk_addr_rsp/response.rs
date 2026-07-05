use apis_saltans_core::ByteSizedVec;
use macaddr::MacAddr8;

/// Successful Network Address Response payload.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum NwkAddrRspResponse {
    /// Single-device response.
    Single {
        /// Remote device IEEE address.
        ieee_addr_remote_dev: MacAddr8,
        /// Remote device network address.
        nwk_addr_remote_dev: u16,
    },
    /// Extended response.
    Extended {
        /// Remote device IEEE address.
        ieee_addr_remote_dev: MacAddr8,
        /// Remote device network address.
        nwk_addr_remote_dev: u16,
        /// Starting index into the associated device list.
        start_index: u8,
        /// Associated device network addresses.
        nwk_addr_assoc_dev_list: Box<ByteSizedVec<u16>>,
    },
}
