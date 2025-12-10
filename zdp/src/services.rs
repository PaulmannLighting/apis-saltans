//! ZDP services.

use le_stream::FromLeStream;
use zigbee::Cluster;

pub use self::bind_req::{BindReq, Destination as BindReqDestination};
pub use self::ieee_addr_req::IeeeAddrReq;
pub use self::mgmt_permit_joining_req::MgmtPermitJoiningReq;
pub use self::nwk_addr_req::{NwkAddrReq, RequestType};
use crate::ParseFrameError;

mod bind_req;
mod ieee_addr_req;
mod mgmt_permit_joining_req;
mod nwk_addr_req;

/// A ZDP client service.
pub trait Service {
    /// The name of the service.
    const NAME: &'static str;
}

/// Available ZDP commands.
// TODO: Implement all commands.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Command {
    /// Bind Request
    BindReq(BindReq),
    /// IEEE Address Request
    IeeeAddrReq(IeeeAddrReq),
    /// Management Permit Joining Request
    MgmtPermitJoiningReq(MgmtPermitJoiningReq),
    /// Network Address Request
    NwkAddrReq(NwkAddrReq),
}

impl Command {
    /// Parses a ZDP command from the given cluster ID and byte iterator.
    pub(crate) fn parse(
        cluster_id: u16,
        bytes: impl Iterator<Item = u8>,
    ) -> Result<Self, ParseFrameError> {
        // TODO: Use a macro to reduce boilerplate.
        match cluster_id {
            BindReq::ID => BindReq::from_le_stream(bytes)
                .map(Self::BindReq)
                .ok_or(ParseFrameError::InsufficientPayload),
            IeeeAddrReq::ID => IeeeAddrReq::from_le_stream(bytes)
                .map(Self::IeeeAddrReq)
                .ok_or(ParseFrameError::InsufficientPayload),
            MgmtPermitJoiningReq::ID => MgmtPermitJoiningReq::from_le_stream(bytes)
                .map(Self::MgmtPermitJoiningReq)
                .ok_or(ParseFrameError::InsufficientPayload),
            NwkAddrReq::ID => NwkAddrReq::from_le_stream(bytes)
                .map(Self::NwkAddrReq)
                .ok_or(ParseFrameError::InsufficientPayload),
            other => Err(ParseFrameError::InvalidCluster(other)),
        }
    }
}
