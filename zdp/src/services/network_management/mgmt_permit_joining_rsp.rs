use std::fmt::Display;

use le_stream::{FromLeStream, ToLeStream};
use zigbee::Cluster;

use crate::{Displayable, Service, Status};

/// Response type for Mgmt Permit Joining Request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct MgmtPermitJoiningRsp {
    status: u8,
}

impl MgmtPermitJoiningRsp {
    /// Creates a new `MgmtPermitJoiningRsp`.
    #[must_use]
    pub const fn new(status: Status) -> Self {
        Self {
            status: status as u8,
        }
    }

    /// Returns the status.
    ///
    /// # Errors
    ///
    /// Returns an error if the status code is invalid.
    pub fn status(self) -> Result<Status, u8> {
        self.status.try_into()
    }
}

impl Cluster for MgmtPermitJoiningRsp {
    const ID: u16 = 0x8036;
}

impl Service for MgmtPermitJoiningRsp {
    const NAME: &'static str = "Mgmt_Permit_Joining_rsp";
}

impl Display for MgmtPermitJoiningRsp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {{ status: {} }}",
            Self::NAME,
            self.status().display()
        )
    }
}
