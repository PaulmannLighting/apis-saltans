//! ZDP services.

pub use self::bind_management::{
    BindManagement, BindReq, BindRsp, ClearAllBindingsReq, ClearAllBindingsRsp, Destination,
    UnbindReq, UnbindRsp,
};
pub use self::device_and_service_discovery::{
    ActiveEpReq, ActiveEpRsp, DeviceAndServiceDiscovery, DeviceAnnce, IeeeAddrReq, IeeeAddrRsp,
    IeeeAddrRspResponse, MatchDescReq, MatchDescRsp, NodeDescReq, NodeDescRsp, NwkAddrReq,
    NwkAddrRsp, NwkAddrRspResponse, ParentAnnce, ParentAnnceRsp, PowerDescReq, PowerDescRsp,
    RequestType, SimpleDescReq, SimpleDescRsp, SystemServerDiscoveryReq, SystemServerDiscoveryRsp,
};
pub use self::network_management::{
    EnhancedNwkUpdateParameters, JoiningPolicy, LeaveReqFlags, MgmtBindReq, MgmtBindRsp,
    MgmtBindRspPayload, MgmtLeaveReq, MgmtLeaveRsp, MgmtLqiReq, MgmtLqiRsp, MgmtLqiRspPayload,
    MgmtNwkBeaconSurveyReq, MgmtNwkBeaconSurveyRsp, MgmtNwkEnhancedUpdateNotify,
    MgmtNwkEnhancedUpdateReq, MgmtNwkIeeeJoiningListReq, MgmtNwkIeeeJoiningListRsp,
    MgmtNwkIeeeJoiningListRspEntries, MgmtNwkIeeeJoiningListRspPayload,
    MgmtNwkUnsolicitedEnhancedUpdateNotify, MgmtNwkUpdateNotify, MgmtNwkUpdateReq,
    MgmtPermitJoiningReq, MgmtPermitJoiningRsp, MgmtRtgReq, MgmtRtgRsp, MgmtRtgRspPayload,
    NetworkManagement, ScanDuration,
};
pub use self::security::{
    Security, SecurityChallengeReq, SecurityChallengeRsp, SecurityDecommissionReq,
    SecurityDecommissionRsp, SecurityGetAuthenticationLevelReq, SecurityGetAuthenticationLevelRsp,
    SecurityGetConfigurationReq, SecurityGetConfigurationRsp,
    SecurityRetrieveAuthenticationTokenReq, SecurityRetrieveAuthenticationTokenRsp,
    SecuritySetConfigurationReq, SecuritySetConfigurationRsp, SecurityStartKeyNegotiationReq,
    SecurityStartKeyNegotiationRsp, SecurityStartKeyUpdateReq, SecurityStartKeyUpdateRsp,
};

mod bind_management;
mod device_and_service_discovery;
mod network_management;
mod security;

/// A ZDP client service.
pub trait Service {
    /// The name of the service.
    const NAME: &'static str;
}

crate::zdp_command_enum! {
    /// Available ZDP commands.
    Command {
        DeviceAndServiceDiscovery,
        BindManagement,
        NetworkManagement,
        Security,
    }
}
