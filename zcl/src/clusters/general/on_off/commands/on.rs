use le_stream::{FromLeStream, ToLeStream};
use zigbee::{ClusterId, ClusterSpecific, Direction};

use crate::Command;

/// Switch a device on.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct On;

impl ClusterSpecific for On {
    const CLUSTER: ClusterId = ClusterId::OnOff;
}

impl Command for On {
    const ID: u8 = 0x01;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl From<On> for crate::Cluster {
    fn from(command: On) -> Self {
        Self::OnOff(command.into())
    }
}
