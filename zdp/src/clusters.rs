use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use zigbee::Cluster;

use crate::{BindReq, IeeeAddrReq, MgmtPermitJoiningReq, NwkAddrReq};

/// Available ZDP clusters.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromPrimitive)]
#[repr(u16)]
pub enum Clusters {
    /// Bind Request.
    BindReq = <BindReq as Cluster>::ID,
    /// IEEE Address Request.
    IeeeAddrReq = <IeeeAddrReq as Cluster>::ID,
    /// Management Permit Joining Request.
    MgmtPermitJoiningReq = <MgmtPermitJoiningReq as Cluster>::ID,
    /// Network Address Request.
    NwkAddrReq = <NwkAddrReq as Cluster>::ID,
}

impl TryFrom<u16> for Clusters {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value).ok_or(value)
    }
}
