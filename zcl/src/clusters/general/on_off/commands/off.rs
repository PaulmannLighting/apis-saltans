use le_stream::{FromLeStream, ToLeStream};
use zigbee::{ClusterId, ClusterSpecific, Direction};

use crate::Command;

/// Switch a device off.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct Off;

impl ClusterSpecific for Off {
    const CLUSTER: ClusterId = ClusterId::OnOff;
}

impl Command for Off {
    const ID: u8 = 0x00;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl From<Off> for crate::Cluster {
    fn from(command: Off) -> Self {
        Self::OnOff(command.into())
    }
}
