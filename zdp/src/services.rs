//! ZDP services.

use le_stream::FromLeStream;
use zigbee::Cluster;

pub use self::bind_management::{
    BindManagement, BindReq, ClearAllBindingsReq, Destination, UnbindReq,
};
pub use self::device_and_service_discovery::{
    ActiveEpReq, DeviceAndServiceDiscovery, DeviceAnnce, IeeeAddrReq, MatchDescReq, NodeDescReq,
    NwkAddrReq, ParentAnnce, PowerDescReq, RequestType, SimpleDescReq, SystemServerDiscoveryReq,
};
pub use self::network_management::{MgmtLqiReq, MgmtPermitJoiningReq, NetworkManagement};

mod bind_management;
mod device_and_service_discovery;
mod network_management;

/// A ZDP client service.
pub trait Service {
    /// The name of the service.
    const NAME: &'static str;
}

/// Available ZDP commands.
// TODO: Implement all commands.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Command {
    /// Device and Service Discovery Commands
    DeviceAndServiceDiscovery(DeviceAndServiceDiscovery),
    /// Bind Management Commands
    BindManagement(BindManagement),
    /// Network Management Commands
    NetworkManagement(NetworkManagement),
}

impl Command {
    /// Parses a ZDP command from the given cluster ID and byte stream.
    pub(crate) fn parse_with_cluster_id<T>(cluster_id: u16, bytes: T) -> Result<Option<Self>, u16>
    where
        T: Iterator<Item = u8>,
    {
        match cluster_id {
            // Device and Service Discovery Commands
            NwkAddrReq::ID => Ok(NwkAddrReq::from_le_stream(bytes)
                .map(DeviceAndServiceDiscovery::NwkAddrReq)
                .map(Self::DeviceAndServiceDiscovery)),
            IeeeAddrReq::ID => Ok(IeeeAddrReq::from_le_stream(bytes)
                .map(DeviceAndServiceDiscovery::IeeeAddrReq)
                .map(Self::DeviceAndServiceDiscovery)),
            NodeDescReq::ID => Ok(NodeDescReq::from_le_stream(bytes)
                .map(DeviceAndServiceDiscovery::NodeDescReq)
                .map(Self::DeviceAndServiceDiscovery)),
            PowerDescReq::ID => Ok(PowerDescReq::from_le_stream(bytes)
                .map(DeviceAndServiceDiscovery::PowerDescReq)
                .map(Self::DeviceAndServiceDiscovery)),
            SimpleDescReq::ID => Ok(SimpleDescReq::from_le_stream(bytes)
                .map(DeviceAndServiceDiscovery::SimpleDescReq)
                .map(Self::DeviceAndServiceDiscovery)),
            ActiveEpReq::ID => Ok(ActiveEpReq::from_le_stream(bytes)
                .map(DeviceAndServiceDiscovery::ActiveEpReq)
                .map(Self::DeviceAndServiceDiscovery)),
            MatchDescReq::ID => Ok(MatchDescReq::from_le_stream(bytes)
                .map(DeviceAndServiceDiscovery::MatchDescReq)
                .map(Self::DeviceAndServiceDiscovery)),
            DeviceAnnce::ID => Ok(DeviceAnnce::from_le_stream(bytes)
                .map(DeviceAndServiceDiscovery::DeviceAnnce)
                .map(Self::DeviceAndServiceDiscovery)),
            ParentAnnce::ID => Ok(ParentAnnce::from_le_stream(bytes)
                .map(DeviceAndServiceDiscovery::ParentAnnce)
                .map(Self::DeviceAndServiceDiscovery)),
            SystemServerDiscoveryReq::ID => Ok(SystemServerDiscoveryReq::from_le_stream(bytes)
                .map(DeviceAndServiceDiscovery::SystemServerDiscoveryReq)
                .map(Self::DeviceAndServiceDiscovery)),
            // Bind Management Commands
            BindReq::ID => Ok(BindReq::from_le_stream(bytes)
                .map(BindManagement::BindReq)
                .map(Self::BindManagement)),
            UnbindReq::ID => Ok(UnbindReq::from_le_stream(bytes)
                .map(BindManagement::UnbindReq)
                .map(Self::BindManagement)),
            ClearAllBindingsReq::ID => Ok(ClearAllBindingsReq::from_le_stream(bytes)
                .map(BindManagement::ClearAllBindingsReq)
                .map(Self::BindManagement)),
            // Network Management Commands
            MgmtLqiReq::ID => Ok(MgmtLqiReq::from_le_stream(bytes)
                .map(NetworkManagement::MgmtLqiReq)
                .map(Self::NetworkManagement)),
            MgmtPermitJoiningReq::ID => Ok(MgmtPermitJoiningReq::from_le_stream(bytes)
                .map(NetworkManagement::MgmtPermitJoiningReq)
                .map(Self::NetworkManagement)),
            other => Err(other),
        }
    }
}
