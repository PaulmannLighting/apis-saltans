//! Network Management Client ZDP Services.

use std::fmt::Display;

pub use self::mgmt_bind_req::MgmtBindReq;
pub use self::mgmt_leave_req::{LeaveReqFlags, MgmtLeaveReq};
pub use self::mgmt_lqi_req::MgmtLqiReq;
pub use self::mgmt_nwk_enhanced_update_req::{
    EnhancedNwkUpdateParameters, MgmtNwkEnhancedUpdateReq,
};
pub use self::mgmt_nwk_ieee_joining_list_req::MgmtNwkIeeeJoiningListReq;
pub use self::mgmt_nwk_update_req::{MgmtNwkUpdateReq, ScanDuration};
pub use self::mgmt_permit_joining_req::MgmtPermitJoiningReq;
pub use self::mgmt_rtg_req::MgmtRtgReq;

mod mgmt_bind_req;
mod mgmt_leave_req;
mod mgmt_lqi_req;
mod mgmt_nwk_enhanced_update_req;
mod mgmt_nwk_ieee_joining_list_req;
mod mgmt_nwk_update_req;
mod mgmt_permit_joining_req;
mod mgmt_rtg_req;

/// Network Management Commands.
// TODO: Implement all commands.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum NetworkManagement {
    /// Management LQI Request
    MgmtLqiReq(MgmtLqiReq),
    /// Management Routing Request
    MgmtRtgReq(MgmtRtgReq),
    /// Management Bind Request
    MgmtBindReq(MgmtBindReq),
    /// Management Leave Request
    MgmtLeaveReq(MgmtLeaveReq),
    /// Management Permit Joining Request
    MgmtPermitJoiningReq(MgmtPermitJoiningReq),
    /// Management Network Update Request
    MgmtNwkUpdateReq(MgmtNwkUpdateReq),
    /// Management Network Enhanced Update Request
    MgmtNwkEnhancedUpdateReq(MgmtNwkEnhancedUpdateReq),
    /// Management Network IEEE Joining List Request.
    MgmtNwkIeeeJoiningListReq(MgmtNwkIeeeJoiningListReq),
}

impl Display for NetworkManagement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MgmtLqiReq(cmd) => cmd.fmt(f),
            Self::MgmtRtgReq(cmd) => cmd.fmt(f),
            Self::MgmtBindReq(cmd) => cmd.fmt(f),
            Self::MgmtLeaveReq(cmd) => cmd.fmt(f),
            Self::MgmtPermitJoiningReq(cmd) => cmd.fmt(f),
            Self::MgmtNwkUpdateReq(cmd) => cmd.fmt(f),
            Self::MgmtNwkEnhancedUpdateReq(cmd) => cmd.fmt(f),
            Self::MgmtNwkIeeeJoiningListReq(cmd) => cmd.fmt(f),
        }
    }
}
