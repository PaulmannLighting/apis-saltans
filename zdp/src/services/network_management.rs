//! Network Management Client ZDP Services.

pub use self::mgmt_lqi_req::MgmtLqiReq;
pub use self::mgmt_permit_joining_req::MgmtPermitJoiningReq;
pub use self::mgmt_rtg_req::MgmtRtgReq;

mod mgmt_lqi_req;
mod mgmt_permit_joining_req;
mod mgmt_rtg_req;

/// Network Management Commands.
// TODO: Implement all commands.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum NetworkManagement {
    /// Management Permit Joining Request
    MgmtPermitJoiningReq(MgmtPermitJoiningReq),
    /// Management LQI Request
    MgmtLqiReq(MgmtLqiReq),
    /// Management Routing Request
    MgmtRtgReq(MgmtRtgReq),
}
