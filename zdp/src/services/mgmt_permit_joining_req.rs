use le_stream::derive::{FromLeStream, ToLeStream};

use crate::Service;

/// Service for management permit joining request.
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct MgmtPermitJoiningReq {
    duration: u8,
    tc_significance: bool,
}

impl MgmtPermitJoiningReq {
    /// Creates a new `MgmtPermitJoiningReq`.
    #[must_use]
    pub const fn new(duration: u8, tc_significance: bool) -> Self {
        Self {
            duration,
            tc_significance,
        }
    }

    /// Returns the duration.
    #[must_use]
    pub const fn duration(&self) -> u8 {
        self.duration
    }

    /// Returns the TC significance.
    #[must_use]
    pub const fn tc_significance(&self) -> bool {
        self.tc_significance
    }
}

impl Service for MgmtPermitJoiningReq {
    const NAME: &'static str = "Mgmt_Permit_Joining_req";
    const CLUSTER_ID: u16 = 0x0036;
}
