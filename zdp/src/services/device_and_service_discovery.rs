//! Device and service discovery ZDP services.

pub use self::active_ep_req::ActiveEpReq;
pub use self::device_annce::DeviceAnnce;
pub use self::ieee_addr_req::IeeeAddrReq;
pub use self::match_desc_req::MatchDescReq;
pub use self::node_desc_req::NodeDescReq;
pub use self::nwk_addr_req::{NwkAddrReq, RequestType};
pub use self::parent_annce::ParentAnnce;
pub use self::power_desc_req::PowerDescReq;
pub use self::simple_desc_req::SimpleDescReq;
pub use self::system_server_discovery_req::SystemServerDiscoveryReq;

mod active_ep_req;
mod device_annce;
mod ieee_addr_req;
mod match_desc_req;
mod node_desc_req;
mod nwk_addr_req;
mod parent_annce;
mod power_desc_req;
mod simple_desc_req;
mod system_server_discovery_req;

/// Device and Service Discovery Commands.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum DeviceAndServiceDiscovery {
    /// Network Address Request
    NwkAddrReq(NwkAddrReq),
    /// IEEE Address Request
    IeeeAddrReq(IeeeAddrReq),
    /// Node Descriptor Request
    NodeDescReq(NodeDescReq),
    /// Power Descriptor Request
    PowerDescReq(PowerDescReq),
    /// Simple Descriptor Request
    SimpleDescReq(SimpleDescReq),
    /// Active Endpoint Request
    ActiveEpReq(ActiveEpReq),
    /// Match Descriptor Request
    MatchDescReq(MatchDescReq),
    /// Device Announcement
    DeviceAnnce(DeviceAnnce),
    /// Parent Announcement
    ParentAnnce(ParentAnnce),
    /// System Server Discovery Request
    SystemServerDiscoveryReq(SystemServerDiscoveryReq),
}
