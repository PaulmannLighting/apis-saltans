use std::fmt::Display;

use le_stream::{FromLeStream, ToLeStream};
use zigbee::Cluster;

use crate::Service;

/// Management LQI Request structure.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct MgmtLqiReq {
    start_index: u8,
}

impl MgmtLqiReq {
    /// Creates a new `MgmtLqiReq`.
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

impl Cluster for MgmtLqiReq {
    const ID: u16 = 0x0031;
}

impl Service for MgmtLqiReq {
    const NAME: &'static str = "Mgmt_Lqi_req";
}

impl Display for MgmtLqiReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {{ start_index: {:#04X} }}",
            Self::NAME,
            self.start_index
        )
    }
}
