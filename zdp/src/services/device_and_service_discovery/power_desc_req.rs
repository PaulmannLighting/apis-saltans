use std::fmt::Display;

use le_stream::{FromLeStream, ToLeStream};
use zigbee::Cluster;

use crate::Service;

/// Power Descriptor Request structure.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct PowerDescReq {
    nwk_addr_of_interest: u16,
}

impl PowerDescReq {
    /// Creates a new `PowerDescReq`.
    #[must_use]
    pub const fn new(nwk_addr_of_interest: u16) -> Self {
        Self {
            nwk_addr_of_interest,
        }
    }

    /// Returns the network address of interest.
    #[must_use]
    pub const fn nwk_addr_of_interest(self) -> u16 {
        self.nwk_addr_of_interest
    }
}

impl Cluster for PowerDescReq {
    const ID: u16 = 0x0003;
}

impl Service for PowerDescReq {
    const NAME: &'static str = "Power_Desc_req";
}

impl Display for PowerDescReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {{ nwk_addr_of_interest: {:#06X} }}",
            Self::NAME,
            self.nwk_addr_of_interest
        )
    }
}

impl From<PowerDescReq> for u16 {
    fn from(req: PowerDescReq) -> Self {
        req.nwk_addr_of_interest
    }
}

impl From<u16> for PowerDescReq {
    fn from(nwk_addr_of_interest: u16) -> Self {
        Self {
            nwk_addr_of_interest,
        }
    }
}
