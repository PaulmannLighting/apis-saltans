use std::fmt::Display;

use le_stream::{FromLeStream, ToLeStream};
use zigbee::Cluster;

use crate::Service;

/// Management Bind Request structure.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct MgmtBindReq {
    start_index: u8,
}

impl MgmtBindReq {
    /// Creates a new `MgmtBindReq`.
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

impl Cluster for MgmtBindReq {
    const ID: u16 = 0x0033;
}

impl Service for MgmtBindReq {
    const NAME: &'static str = "Mgmt_Bind_req";
}

impl Display for MgmtBindReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {{ start_index: {:#04X} }}",
            Self::NAME,
            self.start_index
        )
    }
}
