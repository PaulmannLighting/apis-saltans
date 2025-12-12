use std::fmt::Display;

use le_stream::{FromLeStream, ToLeStream};
use zigbee::Cluster;
use zigbee::node::ServerMask;

use crate::Service;

/// System Server Discovery Request
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, FromLeStream, ToLeStream)]
pub struct SystemServerDiscoveryReq {
    server_mask: ServerMask,
}

impl SystemServerDiscoveryReq {
    /// Creates a new System Server Discovery Request with the specified server mask.
    #[must_use]
    pub const fn new(server_mask: ServerMask) -> Self {
        Self { server_mask }
    }

    /// Returns the server mask of the request.
    #[must_use]
    pub const fn server_mask(self) -> ServerMask {
        self.server_mask
    }
}

impl Cluster for SystemServerDiscoveryReq {
    const ID: u16 = 0x0015;
}

impl Service for SystemServerDiscoveryReq {
    const NAME: &'static str = "System_Server_Discovery_req";
}

impl Display for SystemServerDiscoveryReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {{ server_mask: {} }}", Self::NAME, self.server_mask)
    }
}

impl From<ServerMask> for SystemServerDiscoveryReq {
    fn from(server_mask: ServerMask) -> Self {
        Self::new(server_mask)
    }
}

impl From<SystemServerDiscoveryReq> for ServerMask {
    fn from(req: SystemServerDiscoveryReq) -> Self {
        req.server_mask()
    }
}
