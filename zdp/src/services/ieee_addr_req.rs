use le_stream::derive::{FromLeStream, ToLeStream};

use crate::Service;

/// Request type for IEEE address request.
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct IeeeAddrReq {
    nwk_addr: u16,
    request_type: u8,
    start_index: u8,
}

impl IeeeAddrReq {
    /// Creates a new `IeeeAddrReq`.
    #[must_use]
    pub const fn new(nwk_addr: u16, request_type: u8, start_index: u8) -> Self {
        Self {
            nwk_addr,
            request_type,
            start_index,
        }
    }

    /// Returns the network address.
    #[must_use]
    pub const fn nwk_addr(&self) -> u16 {
        self.nwk_addr
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

impl Service for IeeeAddrReq {
    const NAME: &'static str = "IEEE_addr_req";
    const CLUSTER_ID: u16 = 0x0001;
}
