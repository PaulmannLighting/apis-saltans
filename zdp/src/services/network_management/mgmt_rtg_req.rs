use le_stream::{FromLeStream, ToLeStream};
use zigbee::Cluster;

use crate::Service;

/// Management Routing Table Request structure.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct MgmtRtgReq {
    start_index: u8,
}

impl MgmtRtgReq {
    /// Creates a new `MgmtRtgReq`.
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

impl Cluster for MgmtRtgReq {
    const ID: u16 = 0x0032;
}

impl Service for MgmtRtgReq {
    const NAME: &'static str = "Mgmt_Rtg_req";
}
