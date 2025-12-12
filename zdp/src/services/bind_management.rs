//! Bind, unbind and bind management related ZDP services.

pub use bind_req::{BindReq, Destination};
pub use mgmt_permit_joining_req::MgmtPermitJoiningReq;

mod bind_req;
mod mgmt_permit_joining_req;

/// Bind management commands.
// TODO: Implement all commands.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum BindManagement {
    /// Bind Request
    BindReq(BindReq),
    /// Management Permit Joining Request
    MgmtPermitJoiningReq(MgmtPermitJoiningReq),
}
