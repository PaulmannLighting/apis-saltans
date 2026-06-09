use std::fmt::Display;

use le_stream::{FromLeStream, Prefixed, ToLeStream};
use zigbee::{Cluster, Endpoint};

use crate::{Service, Status};

/// Active Endpoint Response.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct ActiveEpRsp {
    status: u8,
    nwk_addr_of_interest: u16,
    active_eps: Prefixed<u8, Box<[Endpoint]>>,
}

impl ActiveEpRsp {
    /// Attempt to create a new Active Endpoint Response.
    ///
    /// # Errors
    ///
    /// Returns the boxed slice of endpoints if the conversion to a prefixed boxed slice fails.
    pub fn try_new(
        status: Status,
        nwk_addr_of_interest: u16,
        active_eps: &[Endpoint],
    ) -> Result<Self, Box<[Endpoint]>> {
        Box::<[Endpoint]>::from(active_eps)
            .try_into()
            .map(|active_eps| Self {
                status: status.into(),
                nwk_addr_of_interest,
                active_eps,
            })
    }

    /// Return the status of the response.
    pub fn status(&self) -> Result<Status, u8> {
        self.status.try_into()
    }

    /// Return the network address of interest.
    #[must_use]
    pub const fn nwk_addr_of_interest(&self) -> u16 {
        self.nwk_addr_of_interest
    }

    /// Return the active endpoints.
    #[must_use]
    pub fn active_eps(&self) -> &[Endpoint] {
        &self.active_eps
    }
}

impl Cluster for ActiveEpRsp {
    const ID: u16 = 0x8005;
}

impl Service for ActiveEpRsp {
    const NAME: &'static str = "Active_EP_rsp";
}

impl Display for ActiveEpRsp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {{ status: {:#04X}, nwk_addr_of_interest: {:#06X}, active_eps: {:#04X?} }}",
            Self::NAME,
            self.status,
            self.nwk_addr_of_interest,
            self.active_eps
        )
    }
}
