use le_stream::{FromLeStream, ToLeStream};
use zigbee::{ClusterId, ClusterSpecific, Direction};

use crate::{Command, Native};

/// Command to remove all groups from the device's group table.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct RemoveAllGroups;

impl ClusterSpecific for RemoveAllGroups {
    const CLUSTER: ClusterId = ClusterId::Groups;
}

impl Command for RemoveAllGroups {
    const ID: u8 = 0x04;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl Native for RemoveAllGroups {}

impl From<RemoveAllGroups> for crate::Cluster {
    fn from(command: RemoveAllGroups) -> Self {
        Self::Groups(command.into())
    }
}
