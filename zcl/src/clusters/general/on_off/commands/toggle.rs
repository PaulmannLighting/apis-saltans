use le_stream::{FromLeStream, ToLeStream};
use zigbee::{ClusterId, ClusterSpecific, Direction};

use crate::Command;

/// Toggle a device on/off state.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct Toggle;

impl ClusterSpecific for Toggle {
    const CLUSTER: ClusterId = ClusterId::OnOff;
}

impl Command for Toggle {
    const ID: u8 = 0x02;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl From<Toggle> for crate::Cluster {
    fn from(command: Toggle) -> Self {
        Self::OnOff(command.into())
    }
}
