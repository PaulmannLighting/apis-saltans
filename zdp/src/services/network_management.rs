//! Network Management Client ZDP Services.

pub use self::mgmt_permit_joining_req::MgmtPermitJoiningReq;

mod mgmt_permit_joining_req;

/// Network Management Commands.
// TODO: Implement all commands.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum NetworkManagement {
    /// Management Permit Joining Request
    MgmtPermitJoiningReq(MgmtPermitJoiningReq),
}
