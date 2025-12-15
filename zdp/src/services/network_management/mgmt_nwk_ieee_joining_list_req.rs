use std::fmt::Display;

use le_stream::{FromLeStream, ToLeStream};
use zigbee::Cluster;

use crate::Service;

/// Management Network IEEE Joining List Request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct MgmtNwkIeeeJoiningListReq {
    start_index: u8,
}

impl MgmtNwkIeeeJoiningListReq {
    /// Creates a new `MgmtNwkIeeeJoiningListReq`.
    #[must_use]
    pub const fn new(start_index: u8) -> Self {
        Self { start_index }
    }

    /// Returns the start index.
    #[must_use]
    pub const fn start_index(self) -> u8 {
        self.start_index
    }
}

impl Cluster for MgmtNwkIeeeJoiningListReq {
    const ID: u16 = 0x003A;
}

impl Service for MgmtNwkIeeeJoiningListReq {
    const NAME: &'static str = "Mgmt_NWK_IEEE_Joining_List_req";
}

impl Display for MgmtNwkIeeeJoiningListReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {{ start_index: {:#04X} }}",
            Self::NAME,
            self.start_index
        )
    }
}

impl From<u8> for MgmtNwkIeeeJoiningListReq {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl From<MgmtNwkIeeeJoiningListReq> for u8 {
    fn from(value: MgmtNwkIeeeJoiningListReq) -> Self {
        value.start_index
    }
}
