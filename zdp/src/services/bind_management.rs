//! Bind, unbind and bind management related ZDP services.

pub use bind_req::{Address, AddressMode, BindReq, Destination};
pub use mgmt_permit_joining_req::MgmtPermitJoiningReq;
pub use unbind_req::UnbindReq;

mod bind_req;
mod mgmt_permit_joining_req;
mod unbind_req;

/// Bind management commands.
// TODO: Implement all commands.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum BindManagement {
    /// Bind Request
    BindReq(BindReq),
    /// Unbind Request
    UnbindReq(UnbindReq),
    /// Management Permit Joining Request
    MgmtPermitJoiningReq(MgmtPermitJoiningReq),
}
