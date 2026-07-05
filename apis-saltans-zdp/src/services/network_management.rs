//! Network Management Client ZDP Services.

pub use self::mgmt_bind_req::MgmtBindReq;
pub use self::mgmt_leave_req::{LeaveReqFlags, MgmtLeaveReq};
pub use self::mgmt_lqi_req::MgmtLqiReq;
pub use self::mgmt_nwk_beacon_survey_req::MgmtNwkBeaconSurveyReq;
pub use self::mgmt_nwk_enhanced_update_req::{
    EnhancedNwkUpdateParameters, MgmtNwkEnhancedUpdateReq,
};
pub use self::mgmt_nwk_ieee_joining_list_req::MgmtNwkIeeeJoiningListReq;
pub use self::mgmt_nwk_update_req::{MgmtNwkUpdateReq, ScanDuration};
pub use self::mgmt_permit_joining_req::MgmtPermitJoiningReq;
pub use self::mgmt_permit_joining_rsp::MgmtPermitJoiningRsp;
pub use self::mgmt_rtg_req::MgmtRtgReq;

mod mgmt_bind_req;
mod mgmt_leave_req;
mod mgmt_lqi_req;
mod mgmt_nwk_beacon_survey_req;
mod mgmt_nwk_enhanced_update_req;
mod mgmt_nwk_ieee_joining_list_req;
mod mgmt_nwk_update_req;
mod mgmt_permit_joining_req;
mod mgmt_permit_joining_rsp;
mod mgmt_rtg_req;

crate::services::zdp_command_group! {
    /// Network Management Commands.
    NetworkManagement {
        MgmtLqiReq,
        MgmtRtgReq,
        MgmtBindReq,
        MgmtLeaveReq,
        MgmtPermitJoiningReq,
        MgmtNwkUpdateReq,
        MgmtNwkEnhancedUpdateReq,
        MgmtNwkIeeeJoiningListReq,
        MgmtNwkBeaconSurveyReq,
        MgmtPermitJoiningRsp,
    }
}
