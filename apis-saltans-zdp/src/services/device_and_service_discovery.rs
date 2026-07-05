//! Device and service discovery ZDP services.

pub use self::active_ep_req::ActiveEpReq;
pub use self::active_ep_rsp::ActiveEpRsp;
pub use self::device_annce::DeviceAnnce;
pub use self::ieee_addr_req::IeeeAddrReq;
pub use self::match_desc_req::MatchDescReq;
pub use self::match_desc_rsp::MatchDescRsp;
pub use self::node_desc_req::NodeDescReq;
pub use self::node_desc_rsp::NodeDescRsp;
pub use self::nwk_addr_req::{NwkAddrReq, RequestType};
pub use self::parent_annce::ParentAnnce;
pub use self::power_desc_req::PowerDescReq;
pub use self::simple_desc_req::SimpleDescReq;
pub use self::simple_desc_rsp::SimpleDescRsp;
pub use self::system_server_discovery_req::SystemServerDiscoveryReq;

mod active_ep_req;
mod active_ep_rsp;
mod device_annce;
mod ieee_addr_req;
mod match_desc_req;
mod match_desc_rsp;
mod node_desc_req;
mod node_desc_rsp;
mod nwk_addr_req;
mod parent_annce;
mod power_desc_req;
mod simple_desc_req;
mod simple_desc_rsp;
mod system_server_discovery_req;

crate::zdp_command_group! {
    /// Device and Service Discovery Commands.
    DeviceAndServiceDiscovery {
        NwkAddrReq,
        IeeeAddrReq,
        NodeDescReq,
        NodeDescRsp,
        PowerDescReq,
        SimpleDescReq,
        SimpleDescRsp,
        ActiveEpReq,
        ActiveEpRsp,
        MatchDescReq,
        MatchDescRsp,
        DeviceAnnce,
        ParentAnnce,
        SystemServerDiscoveryReq,
    }
}
