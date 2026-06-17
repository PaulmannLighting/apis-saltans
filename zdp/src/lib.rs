//! Zigbee device profile (ZDP) library.
//!
//! TODO: Implement all services and an appropriate trait to send and receive the respective frames.

extern crate core;

pub use status::{Displayable, Status};

pub use self::frame::Frame;
pub use self::services::{
    ActiveEpReq, ActiveEpRsp, BindManagement, BindReq, BindRsp, ClearAllBindingsReq, Command,
    Destination, DeviceAndServiceDiscovery, DeviceAnnce, EnhancedNwkUpdateParameters, IeeeAddrReq,
    LeaveReqFlags, MatchDescReq, MatchDescRsp, MgmtBindReq, MgmtLeaveReq, MgmtLqiReq,
    MgmtNwkBeaconSurveyReq, MgmtNwkEnhancedUpdateReq, MgmtNwkIeeeJoiningListReq, MgmtNwkUpdateReq,
    MgmtPermitJoiningReq, MgmtPermitJoiningRsp, MgmtRtgReq, NetworkManagement, NodeDescReq,
    NodeDescRsp, NwkAddrReq, ParentAnnce, PowerDescReq, RequestType, ScanDuration, Service,
    SimpleDescReq, SimpleDescRsp, SystemServerDiscoveryReq, UnbindReq,
};
pub use self::simple_descriptor::{AppFlags, Clusters, SimpleDescriptor};

mod frame;
mod services;
mod simple_descriptor;
mod status;

/// Type alias for a byte-sized heapless vector.
type ByteSizedVec<T> = heapless::Vec<T, { u8::MAX as usize }, u8>;
