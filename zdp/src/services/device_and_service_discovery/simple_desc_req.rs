use std::fmt::Display;

use le_stream::{FromLeStream, ToLeStream};
use zigbee::Cluster;

use crate::Service;

/// Simple Descriptor Request structure.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct SimpleDescReq {
    nwk_address_of_interest: u16,
    endpoint: u8,
}

impl SimpleDescReq {
    /// Creates a new `SimpleDescReq`.
    #[must_use]
    pub const fn new(nwk_address_of_interest: u16, endpoint: u8) -> Self {
        Self {
            nwk_address_of_interest,
            endpoint,
        }
    }

    /// Returns the network address of interest.
    #[must_use]
    pub const fn nwk_address_of_interest(self) -> u16 {
        self.nwk_address_of_interest
    }

    /// Returns the endpoint.
    #[must_use]
    pub const fn endpoint(self) -> u8 {
        self.endpoint
    }
}

impl Cluster for SimpleDescReq {
    const ID: u16 = 0x0004;
}

impl Service for SimpleDescReq {
    const NAME: &'static str = "Simple_Desc_req";
}

impl Display for SimpleDescReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {{ nwk_address_of_interest: {:#06X}, endpoint: {:#04X} }}",
            Self::NAME,
            self.nwk_address_of_interest,
            self.endpoint
        )
    }
}
