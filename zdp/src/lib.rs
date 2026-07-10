//! Zigbee Device Profile (ZDP) frame and service command models.
//!
//! The crate provides typed ZDP request/response payloads, grouped service enums, a unified
//! [`Command`] enum, command dispatch by cluster ID, and sequence-numbered [`Frame`] wrappers.
//! Implemented service groups currently include Device and Service Discovery, Bind Management,
//! Network Management, and Security.

extern crate core;

pub use status::{Displayable, Status};

pub use self::frame::{Frame, ParseFrameError};
pub use self::services::{
    ActiveEpReq, ActiveEpRsp, BindManagement, BindReq, BindRsp, ClearAllBindingsReq,
    ClearAllBindingsRsp, Command, Destination, DeviceAndServiceDiscovery, DeviceAnnce,
    EnhancedNwkUpdateParameters, IeeeAddrReq, IeeeAddrRsp, IeeeAddrRspResponse, JoiningPolicy,
    LeaveReqFlags, MatchDescReq, MatchDescRsp, MgmtBindReq, MgmtBindRsp, MgmtBindRspPayload,
    MgmtLeaveReq, MgmtLeaveRsp, MgmtLqiReq, MgmtLqiRsp, MgmtLqiRspPayload, MgmtNwkBeaconSurveyReq,
    MgmtNwkBeaconSurveyRsp, MgmtNwkEnhancedUpdateNotify, MgmtNwkEnhancedUpdateReq,
    MgmtNwkIeeeJoiningListReq, MgmtNwkIeeeJoiningListRsp, MgmtNwkIeeeJoiningListRspEntries,
    MgmtNwkIeeeJoiningListRspPayload, MgmtNwkUnsolicitedEnhancedUpdateNotify, MgmtNwkUpdateNotify,
    MgmtNwkUpdateReq, MgmtPermitJoiningReq, MgmtPermitJoiningRsp, MgmtRtgReq, MgmtRtgRsp,
    MgmtRtgRspPayload, NetworkManagement, NodeDescReq, NodeDescRsp, NwkAddrReq, NwkAddrRsp,
    NwkAddrRspResponse, ParentAnnce, ParentAnnceRsp, PowerDescReq, PowerDescRsp, RequestType,
    ScanDuration, Security, SecurityChallengeReq, SecurityChallengeRsp, SecurityDecommissionReq,
    SecurityDecommissionRsp, SecurityGetAuthenticationLevelReq, SecurityGetAuthenticationLevelRsp,
    SecurityGetConfigurationReq, SecurityGetConfigurationRsp,
    SecurityRetrieveAuthenticationTokenReq, SecurityRetrieveAuthenticationTokenRsp,
    SecuritySetConfigurationReq, SecuritySetConfigurationRsp, SecurityStartKeyNegotiationReq,
    SecurityStartKeyNegotiationRsp, SecurityStartKeyUpdateReq, SecurityStartKeyUpdateRsp, Service,
    SimpleDescReq, SimpleDescRsp, SystemServerDiscoveryReq, SystemServerDiscoveryRsp, UnbindReq,
    UnbindRsp,
};
pub use self::simple_descriptor::{AppFlags, Clusters, SimpleDescriptor};

mod frame;
mod macros;
mod services;
mod simple_descriptor;
mod status;

pub(crate) use self::macros::{zdp_command, zdp_command_enum, zdp_command_group};

/// Bit mask that marks a ZDP cluster ID as a response cluster.
///
/// ZDP response cluster IDs are formed by setting bit 15 on the corresponding
/// request cluster ID.
pub const CLUSTER_ID_RESPONSE_MASK: u16 = 0x8000;
