//! ZDP services.

use le_stream::{FromLeStream, FromLeStreamTagged};
use zigbee::Cluster;

pub use self::active_ep_req::ActiveEpReq;
pub use self::bind_req::{BindReq, Destination as BindReqDestination};
pub use self::device_annce::DeviceAnnce;
pub use self::ieee_addr_req::IeeeAddrReq;
pub use self::match_desc_req::MatchDescReq;
pub use self::mgmt_permit_joining_req::MgmtPermitJoiningReq;
pub use self::node_desc_req::NodeDescReq;
pub use self::nwk_addr_req::{NwkAddrReq, RequestType};
pub use self::power_desc_req::PowerDescReq;
pub use self::simple_desc_req::SimpleDescReq;

mod active_ep_req;
mod bind_req;
mod device_annce;
mod ieee_addr_req;
mod match_desc_req;
mod mgmt_permit_joining_req;
mod node_desc_req;
mod nwk_addr_req;
mod power_desc_req;
mod simple_desc_req;

/// A ZDP client service.
pub trait Service {
    /// The name of the service.
    const NAME: &'static str;
}

/// Available ZDP commands.
// TODO: Implement all commands.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Command {
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
    /// Bind Request
    BindReq(BindReq),
    /// Management Permit Joining Request
    MgmtPermitJoiningReq(MgmtPermitJoiningReq),
}

impl FromLeStreamTagged for Command {
    type Tag = u16;

    fn from_le_stream_tagged<T>(cluster_id: Self::Tag, bytes: T) -> Result<Option<Self>, Self::Tag>
    where
        T: Iterator<Item = u8>,
    {
        match cluster_id {
            NwkAddrReq::ID => Ok(NwkAddrReq::from_le_stream(bytes).map(Self::NwkAddrReq)),
            IeeeAddrReq::ID => Ok(IeeeAddrReq::from_le_stream(bytes).map(Self::IeeeAddrReq)),
            NodeDescReq::ID => Ok(NodeDescReq::from_le_stream(bytes).map(Self::NodeDescReq)),
            PowerDescReq::ID => Ok(PowerDescReq::from_le_stream(bytes).map(Self::PowerDescReq)),
            SimpleDescReq::ID => Ok(SimpleDescReq::from_le_stream(bytes).map(Self::SimpleDescReq)),
            ActiveEpReq::ID => Ok(ActiveEpReq::from_le_stream(bytes).map(Self::ActiveEpReq)),
            MatchDescReq::ID => Ok(MatchDescReq::from_le_stream(bytes).map(Self::MatchDescReq)),
            DeviceAnnce::ID => Ok(DeviceAnnce::from_le_stream(bytes).map(Self::DeviceAnnce)),
            BindReq::ID => Ok(BindReq::from_le_stream(bytes).map(Self::BindReq)),
            MgmtPermitJoiningReq::ID => {
                Ok(MgmtPermitJoiningReq::from_le_stream(bytes).map(Self::MgmtPermitJoiningReq))
            }
            other => Err(other),
        }
    }
}
