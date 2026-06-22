//! Device and service discovery ZDP services.

use std::fmt::Display;

use zigbee::Cluster;

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

/// Device and Service Discovery Commands.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum DeviceAndServiceDiscovery {
    /// Network Address Request
    NwkAddrReq(NwkAddrReq),

    /// IEEE Address Request
    IeeeAddrReq(IeeeAddrReq),

    /// Node Descriptor Request
    NodeDescReq(NodeDescReq),

    /// Node Descriptor Response
    NodeDescRsp(NodeDescRsp),

    /// Power Descriptor Request
    PowerDescReq(PowerDescReq),

    /// Simple Descriptor Request
    SimpleDescReq(SimpleDescReq),

    /// Simple Descriptor Response
    SimpleDescRsp(SimpleDescRsp),

    /// Active Endpoint Request
    ActiveEpReq(ActiveEpReq),

    /// Active Endpoint Request
    ActiveEpRsp(ActiveEpRsp),

    /// Match Descriptor Request
    MatchDescReq(MatchDescReq),

    /// Match Descriptor Response.
    MatchDescRsp(MatchDescRsp),

    /// Device Announcement
    DeviceAnnce(DeviceAnnce),

    /// Parent Announcement
    ParentAnnce(Box<ParentAnnce>),

    /// System Server Discovery Request
    SystemServerDiscoveryReq(SystemServerDiscoveryReq),
}

impl DeviceAndServiceDiscovery {
    /// Return the cluster ID of the command.
    #[must_use]
    pub const fn cluster_id(&self) -> u16 {
        match self {
            Self::MatchDescRsp(_) => <MatchDescRsp as Cluster>::ID,
            Self::NwkAddrReq(_) => <NwkAddrReq as Cluster>::ID,
            Self::IeeeAddrReq(_) => <IeeeAddrReq as Cluster>::ID,
            Self::NodeDescReq(_) => <NodeDescReq as Cluster>::ID,
            Self::NodeDescRsp(_) => <NodeDescRsp as Cluster>::ID,
            Self::PowerDescReq(_) => <PowerDescReq as Cluster>::ID,
            Self::SimpleDescReq(_) => <SimpleDescReq as Cluster>::ID,
            Self::SimpleDescRsp(_) => <SimpleDescRsp as Cluster>::ID,
            Self::ActiveEpReq(_) => <ActiveEpReq as Cluster>::ID,
            Self::ActiveEpRsp(_) => <ActiveEpRsp as Cluster>::ID,
            Self::MatchDescReq(_) => <MatchDescReq as Cluster>::ID,
            Self::DeviceAnnce(_) => <DeviceAnnce as Cluster>::ID,
            Self::ParentAnnce(_) => <ParentAnnce as Cluster>::ID,
            Self::SystemServerDiscoveryReq(_) => <SystemServerDiscoveryReq as Cluster>::ID,
        }
    }
}

impl Display for DeviceAndServiceDiscovery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NwkAddrReq(cmd) => cmd.fmt(f),
            Self::IeeeAddrReq(cmd) => cmd.fmt(f),
            Self::NodeDescReq(cmd) => cmd.fmt(f),
            Self::NodeDescRsp(cmd) => cmd.fmt(f),
            Self::PowerDescReq(cmd) => cmd.fmt(f),
            Self::SimpleDescReq(cmd) => cmd.fmt(f),
            Self::SimpleDescRsp(cmd) => cmd.fmt(f),
            Self::ActiveEpReq(cmd) => cmd.fmt(f),
            Self::ActiveEpRsp(cmd) => cmd.fmt(f),
            Self::MatchDescReq(cmd) => cmd.fmt(f),
            Self::MatchDescRsp(cmd) => cmd.fmt(f),
            Self::DeviceAnnce(cmd) => cmd.fmt(f),
            Self::ParentAnnce(cmd) => cmd.fmt(f),
            Self::SystemServerDiscoveryReq(cmd) => cmd.fmt(f),
        }
    }
}

impl From<NwkAddrReq> for DeviceAndServiceDiscovery {
    fn from(cmd: NwkAddrReq) -> Self {
        Self::NwkAddrReq(cmd)
    }
}

impl From<IeeeAddrReq> for DeviceAndServiceDiscovery {
    fn from(cmd: IeeeAddrReq) -> Self {
        Self::IeeeAddrReq(cmd)
    }
}

impl From<NodeDescReq> for DeviceAndServiceDiscovery {
    fn from(cmd: NodeDescReq) -> Self {
        Self::NodeDescReq(cmd)
    }
}

impl From<NodeDescRsp> for DeviceAndServiceDiscovery {
    fn from(cmd: NodeDescRsp) -> Self {
        Self::NodeDescRsp(cmd)
    }
}

impl From<PowerDescReq> for DeviceAndServiceDiscovery {
    fn from(cmd: PowerDescReq) -> Self {
        Self::PowerDescReq(cmd)
    }
}

impl From<SimpleDescReq> for DeviceAndServiceDiscovery {
    fn from(cmd: SimpleDescReq) -> Self {
        Self::SimpleDescReq(cmd)
    }
}

impl From<SimpleDescRsp> for DeviceAndServiceDiscovery {
    fn from(cmd: SimpleDescRsp) -> Self {
        Self::SimpleDescRsp(cmd)
    }
}

impl From<ActiveEpReq> for DeviceAndServiceDiscovery {
    fn from(cmd: ActiveEpReq) -> Self {
        Self::ActiveEpReq(cmd)
    }
}

impl From<ActiveEpRsp> for DeviceAndServiceDiscovery {
    fn from(cmd: ActiveEpRsp) -> Self {
        Self::ActiveEpRsp(cmd)
    }
}

impl From<MatchDescReq> for DeviceAndServiceDiscovery {
    fn from(cmd: MatchDescReq) -> Self {
        Self::MatchDescReq(cmd)
    }
}

impl From<MatchDescRsp> for DeviceAndServiceDiscovery {
    fn from(cmd: MatchDescRsp) -> Self {
        Self::MatchDescRsp(cmd)
    }
}

impl From<DeviceAnnce> for DeviceAndServiceDiscovery {
    fn from(cmd: DeviceAnnce) -> Self {
        Self::DeviceAnnce(cmd)
    }
}

impl From<ParentAnnce> for DeviceAndServiceDiscovery {
    fn from(cmd: ParentAnnce) -> Self {
        Self::ParentAnnce(cmd.into())
    }
}

impl From<SystemServerDiscoveryReq> for DeviceAndServiceDiscovery {
    fn from(cmd: SystemServerDiscoveryReq) -> Self {
        Self::SystemServerDiscoveryReq(cmd)
    }
}
