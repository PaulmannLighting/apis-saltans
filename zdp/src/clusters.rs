use zigbee::Cluster;

use crate::{BindReq, IeeeAddrReq, MgmtPermitJoiningReq, NwkAddrReq};

/// Available ZDP clusters.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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
