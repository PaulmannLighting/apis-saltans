//! Device and service discovery ZDP services.

pub use self::active_ep_req::ActiveEpReq;
pub use self::active_ep_rsp::ActiveEpRsp;
pub use self::device_annce::DeviceAnnce;
pub use self::ieee_addr_req::IeeeAddrReq;
pub use self::ieee_addr_rsp::IeeeAddrRsp;
pub use self::match_desc_req::MatchDescReq;
pub use self::match_desc_rsp::MatchDescRsp;
pub use self::node_desc_req::NodeDescReq;
pub use self::node_desc_rsp::NodeDescRsp;
pub use self::nwk_addr_req::{NwkAddrReq, RequestType};
pub use self::nwk_addr_rsp::NwkAddrRsp;
pub use self::parent_annce::ParentAnnce;
pub use self::parent_annce_rsp::ParentAnnceRsp;
pub use self::power_desc_req::PowerDescReq;
pub use self::power_desc_rsp::PowerDescRsp;
pub use self::simple_desc_req::SimpleDescReq;
pub use self::simple_desc_rsp::SimpleDescRsp;
pub use self::system_server_discovery_req::SystemServerDiscoveryReq;
pub use self::system_server_discovery_rsp::SystemServerDiscoveryRsp;

mod active_ep_req;
mod active_ep_rsp;
mod device_annce;
mod ieee_addr_req;
mod ieee_addr_rsp;
mod match_desc_req;
mod match_desc_rsp;
mod node_desc_req;
mod node_desc_rsp;
mod nwk_addr_req;
mod nwk_addr_rsp;
mod parent_annce;
mod parent_annce_rsp;
mod power_desc_req;
mod power_desc_rsp;
mod simple_desc_req;
mod simple_desc_rsp;
mod system_server_discovery_req;
mod system_server_discovery_rsp;

crate::zdp_command_group! {
    /// Device and Service Discovery Commands.
    DeviceAndServiceDiscovery {
        NwkAddrReq,
        NwkAddrRsp,
        IeeeAddrReq,
        IeeeAddrRsp,
        NodeDescReq,
        NodeDescRsp,
        PowerDescReq,
        PowerDescRsp,
        SimpleDescReq,
        SimpleDescRsp,
        ActiveEpReq,
        ActiveEpRsp,
        MatchDescReq,
        MatchDescRsp,
        DeviceAnnce,
        ParentAnnce,
        ParentAnnceRsp,
        SystemServerDiscoveryReq,
        SystemServerDiscoveryRsp,
    }
}
