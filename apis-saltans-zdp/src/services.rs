//! ZDP services.

pub use self::bind_management::{
    BindManagement, BindReq, BindRsp, ClearAllBindingsReq, Destination, UnbindReq,
};
pub use self::device_and_service_discovery::{
    ActiveEpReq, ActiveEpRsp, DeviceAndServiceDiscovery, DeviceAnnce, IeeeAddrReq, MatchDescReq,
    MatchDescRsp, NodeDescReq, NodeDescRsp, NwkAddrReq, ParentAnnce, PowerDescReq, RequestType,
    SimpleDescReq, SimpleDescRsp, SystemServerDiscoveryReq,
};
pub use self::network_management::{
    EnhancedNwkUpdateParameters, LeaveReqFlags, MgmtBindReq, MgmtLeaveReq, MgmtLqiReq,
    MgmtNwkBeaconSurveyReq, MgmtNwkEnhancedUpdateReq, MgmtNwkIeeeJoiningListReq, MgmtNwkUpdateReq,
    MgmtPermitJoiningReq, MgmtPermitJoiningRsp, MgmtRtgReq, NetworkManagement, ScanDuration,
};

mod bind_management;
mod device_and_service_discovery;
mod network_management;

pub(crate) use crate::macros::{zdp_command, zdp_command_enum, zdp_command_group};

/// A ZDP client service.
pub trait Service {
    /// The name of the service.
    const NAME: &'static str;
}

zdp_command_enum! {
    /// Available ZDP commands.
    Command {
        DeviceAndServiceDiscovery,
        BindManagement,
        NetworkManagement,
    }
}
