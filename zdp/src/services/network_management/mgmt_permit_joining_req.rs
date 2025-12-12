use std::fmt::Display;

use le_stream::{FromLeStream, ToLeStream};
use zigbee::Cluster;
use zigbee::types::tlv::Tlv;

use crate::Service;

/// Service for management permit joining request.
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct MgmtPermitJoiningReq {
    duration: u8,
    tc_significance: bool,
    tlv_data: Vec<Tlv>,
}

impl MgmtPermitJoiningReq {
    /// Creates a new `MgmtPermitJoiningReq`.
    #[must_use]
    pub const fn new(duration: u8, tc_significance: bool, tlv_data: Vec<Tlv>) -> Self {
        Self {
            duration,
            tc_significance,
            tlv_data,
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

    /// Returns the TLV data.
    #[must_use]
    pub fn tlv_data(&self) -> &[Tlv] {
        &self.tlv_data
    }
}

impl Cluster for MgmtPermitJoiningReq {
    const ID: u16 = 0x0036;
}

impl Service for MgmtPermitJoiningReq {
    const NAME: &'static str = "Mgmt_Permit_Joining_req";
}

impl Display for MgmtPermitJoiningReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {{ duration: {:#04X}, tc_significance: {}, tlv_data: {:?} }}",
            Self::NAME,
            self.duration,
            self.tc_significance,
            self.tlv_data
        )
    }
}
