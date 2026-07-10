//! Network Management Client ZDP Services.

pub use self::mgmt_bind_req::MgmtBindReq;
pub use self::mgmt_bind_rsp::{MgmtBindRsp, MgmtBindRspPayload};
pub use self::mgmt_leave_req::{LeaveReqFlags, MgmtLeaveReq};
pub use self::mgmt_leave_rsp::MgmtLeaveRsp;
pub use self::mgmt_lqi_req::MgmtLqiReq;
pub use self::mgmt_lqi_rsp::{MgmtLqiRsp, MgmtLqiRspPayload};
pub use self::mgmt_nwk_beacon_survey_req::MgmtNwkBeaconSurveyReq;
pub use self::mgmt_nwk_beacon_survey_rsp::MgmtNwkBeaconSurveyRsp;
pub use self::mgmt_nwk_enhanced_update_notify::MgmtNwkEnhancedUpdateNotify;
pub use self::mgmt_nwk_enhanced_update_req::{
    EnhancedNwkUpdateParameters, MgmtNwkEnhancedUpdateReq,
};
pub use self::mgmt_nwk_ieee_joining_list_req::MgmtNwkIeeeJoiningListReq;
pub use self::mgmt_nwk_ieee_joining_list_rsp::{
    JoiningPolicy, MgmtNwkIeeeJoiningListRsp, MgmtNwkIeeeJoiningListRspEntries,
    MgmtNwkIeeeJoiningListRspPayload,
};
pub use self::mgmt_nwk_unsolicited_enhanced_update_notify::MgmtNwkUnsolicitedEnhancedUpdateNotify;
pub use self::mgmt_nwk_update_notify::MgmtNwkUpdateNotify;
pub use self::mgmt_nwk_update_req::{MgmtNwkUpdateReq, ScanDuration};
pub use self::mgmt_permit_joining_req::MgmtPermitJoiningReq;
pub use self::mgmt_permit_joining_rsp::MgmtPermitJoiningRsp;
pub use self::mgmt_rtg_req::MgmtRtgReq;
pub use self::mgmt_rtg_rsp::{MgmtRtgRsp, MgmtRtgRspPayload};

mod mgmt_bind_req;
mod mgmt_bind_rsp;
mod mgmt_leave_req;
mod mgmt_leave_rsp;
mod mgmt_lqi_req;
mod mgmt_lqi_rsp;
mod mgmt_nwk_beacon_survey_req;
mod mgmt_nwk_beacon_survey_rsp;
mod mgmt_nwk_enhanced_update_notify;
mod mgmt_nwk_enhanced_update_req;
mod mgmt_nwk_ieee_joining_list_req;
mod mgmt_nwk_ieee_joining_list_rsp;
mod mgmt_nwk_unsolicited_enhanced_update_notify;
mod mgmt_nwk_update_notify;
mod mgmt_nwk_update_req;
mod mgmt_permit_joining_req;
mod mgmt_permit_joining_rsp;
mod mgmt_rtg_req;
mod mgmt_rtg_rsp;

crate::zdp_command_group! {
    /// Network Management Commands.
    NetworkManagement {
        MgmtLqiReq,
        MgmtLqiRsp,
        MgmtRtgReq,
        MgmtRtgRsp,
        MgmtBindReq,
        MgmtBindRsp,
        MgmtLeaveReq,
        MgmtLeaveRsp,
        MgmtPermitJoiningReq,
        MgmtNwkUpdateReq,
        MgmtNwkUpdateNotify,
        MgmtNwkEnhancedUpdateReq,
        MgmtNwkEnhancedUpdateNotify,
        MgmtNwkIeeeJoiningListReq,
        MgmtNwkIeeeJoiningListRsp,
        MgmtNwkUnsolicitedEnhancedUpdateNotify,
        MgmtNwkBeaconSurveyReq,
        MgmtNwkBeaconSurveyRsp,
        MgmtPermitJoiningRsp,
    }
}
