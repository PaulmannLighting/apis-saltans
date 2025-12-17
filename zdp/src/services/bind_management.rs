//! Bind, unbind and bind management related ZDP services.

use std::fmt::Display;

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
