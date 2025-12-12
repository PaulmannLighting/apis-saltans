use le_stream::{FromLeStream, ToLeStream};
use zigbee::Cluster;

use crate::Service;

/// Request type for IEEE address request.
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct IeeeAddrReq {
    nwk_addr_of_interest: u16,
    request_type: u8,
    start_index: u8,
}

impl IeeeAddrReq {
    /// Creates a new `IeeeAddrReq`.
    #[must_use]
    pub const fn new(nwk_addr_of_interest: u16, request_type: u8, start_index: u8) -> Self {
        Self {
            nwk_addr_of_interest,
            request_type,
            start_index,
        }
    }

    /// Returns the network address of interest.
    #[must_use]
    pub const fn nwk_addr_of_interest(&self) -> u16 {
        self.nwk_addr_of_interest
    }

    /// Returns the request type.
    #[must_use]
    pub const fn request_type(&self) -> u8 {
        self.request_type
    }

    /// Returns the start index.
    #[must_use]
    pub const fn start_index(&self) -> u8 {
        self.start_index
    }
}

impl Cluster for IeeeAddrReq {
    const ID: u16 = 0x0001;
}

impl Service for IeeeAddrReq {
    const NAME: &'static str = "IEEE_addr_req";
}
