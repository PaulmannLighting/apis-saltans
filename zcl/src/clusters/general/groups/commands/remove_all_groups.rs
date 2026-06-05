use le_stream::{FromLeStream, ToLeStream};
use zigbee::{Cluster, ClusterId, Direction};

use crate::{Command, Native};

/// Command to remove all groups from the device's group table.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct RemoveAllGroups;

impl Cluster for RemoveAllGroups {
    const ID: u16 = ClusterId::Groups.as_u16();
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
