//! Bind, unbind, and bind management related ZDP services.

use std::fmt::Display;

use apis_saltans_core::Cluster;

pub use self::bind_req::{Address, AddressMode, BindReq, Destination};
pub use self::bind_rsp::BindRsp;
pub use self::clear_all_bindings_req::ClearAllBindingsReq;
pub use self::unbind_req::UnbindReq;

mod bind_req;
mod bind_rsp;
mod clear_all_bindings_req;
mod unbind_req;

/// Bind management commands.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum BindManagement {
    /// Bind Request
    BindReq(BindReq),

    /// Bind Response
    BindRsp(BindRsp),

    /// Unbind Request
    UnbindReq(UnbindReq),

    /// Clear All Bindings Request
    ClearAllBindingsReq(ClearAllBindingsReq),
}

impl BindManagement {
    /// Returns the cluster ID of the command.
    #[must_use]
    pub const fn cluster_id(&self) -> u16 {
        match self {
            Self::BindReq(_) => <BindReq as Cluster>::ID,
            Self::BindRsp(_) => <BindRsp as Cluster>::ID,
            Self::UnbindReq(_) => <UnbindReq as Cluster>::ID,
            Self::ClearAllBindingsReq(_) => <ClearAllBindingsReq as Cluster>::ID,
        }
    }
}

impl Display for BindManagement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BindReq(cmd) => cmd.fmt(f),
            Self::BindRsp(cmd) => cmd.fmt(f),
            Self::UnbindReq(cmd) => cmd.fmt(f),
            Self::ClearAllBindingsReq(cmd) => cmd.fmt(f),
        }
    }
}

impl From<BindReq> for BindManagement {
    fn from(cmd: BindReq) -> Self {
        Self::BindReq(cmd)
    }
}

impl From<BindRsp> for BindManagement {
    fn from(cmd: BindRsp) -> Self {
        Self::BindRsp(cmd)
    }
}

impl From<UnbindReq> for BindManagement {
    fn from(cmd: UnbindReq) -> Self {
        Self::UnbindReq(cmd)
    }
}

impl From<ClearAllBindingsReq> for BindManagement {
    fn from(cmd: ClearAllBindingsReq) -> Self {
        Self::ClearAllBindingsReq(cmd)
    }
}
