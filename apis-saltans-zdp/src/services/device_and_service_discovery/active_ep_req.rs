use std::fmt::Display;

use apis_saltans_core::{Cluster, ExpectResponse};
use le_stream::{FromLeStream, ToLeStream};

use crate::{ActiveEpRsp, Command, Service};

/// Active Endpoint Request
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, ToLeStream, FromLeStream)]
pub struct ActiveEpReq {
    nwk_addr_of_interest: u16,
}

impl ActiveEpReq {
    /// Creates a new `ActiveEpReq` with the given network address of interest.
    #[must_use]
    pub const fn new(nwk_addr_of_interest: u16) -> Self {
        Self {
            nwk_addr_of_interest,
        }
    }

    /// Returns the network address of interest.
    #[must_use]
    pub const fn nwk_addr_of_interest(&self) -> u16 {
        self.nwk_addr_of_interest
    }
}

impl Cluster for ActiveEpReq {
    const ID: u16 = 0x0005;
}

impl Service for ActiveEpReq {
    const NAME: &'static str = "Active_EP_req";
}

impl ExpectResponse<Command> for ActiveEpReq {
    type Response = ActiveEpRsp;
}

impl Display for ActiveEpReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {{ nwk_addr_of_interest: {:#06X} }}",
            Self::NAME,
            self.nwk_addr_of_interest
        )
    }
}

impl From<ActiveEpReq> for Command {
    fn from(active_ep_req: ActiveEpReq) -> Self {
        Self::DeviceAndServiceDiscovery(active_ep_req.into())
    }
}
