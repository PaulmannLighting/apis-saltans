use std::fmt::Display;

use le_stream::{FromLeStream, ToLeStream};
use zigbee::Cluster;

use crate::{Service, Status};

/// Binding response.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct BindRsp {
    status: u8,
}

impl BindRsp {
    /// Creates a new `BindRsp`.
    #[must_use]
    pub const fn new(status: Status) -> Self {
        Self {
            status: status as u8,
        }
    }

    /// Returns the status.
    pub fn status(self) -> Result<Status, u8> {
        self.status.try_into()
    }
}

impl Cluster for BindRsp {
    const ID: u16 = 0x8021;
}

impl Service for BindRsp {
    const NAME: &'static str = "Bind_rsp";
}

impl Display for BindRsp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {{ status: {:#04X} }}", Self::NAME, self.status)
    }
}
