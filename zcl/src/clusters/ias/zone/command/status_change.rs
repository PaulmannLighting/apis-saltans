use le_stream::{FromLeStream, ToLeStream};
use zigbee::types::Uint16;
use zigbee::{ClusterId, ClusterSpecific};

use crate::ias::zone::Status;
use crate::{Cluster, Command};

/// Zone status change attributes.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, FromLeStream, ToLeStream)]
pub struct StatusChange {
    status: Status,
    extended_status: u8,
    zone_id: u8,
    delay: Uint16,
}

impl ClusterSpecific for StatusChange {
    const CLUSTER: ClusterId = ClusterId::IasZone;
}

impl Command for StatusChange {
    const ID: u8 = 0x00;
    const DIRECTION: zigbee::Direction = zigbee::Direction::ServerToClient;
}

impl From<StatusChange> for Cluster {
    fn from(value: StatusChange) -> Self {
        Self::IasZone(value.into())
    }
}
