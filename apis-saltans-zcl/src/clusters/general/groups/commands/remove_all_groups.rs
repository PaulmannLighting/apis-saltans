use le_stream::{FromLeStream, ToLeStream};
use apis_saltans_core::{ClusterId, Cluster, Direction};

use crate::Command;

/// Command to remove all groups from the device's group table.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct RemoveAllGroups;

impl Cluster<ClusterId> for RemoveAllGroups {
    const ID: ClusterId = ClusterId::Groups;
}

impl Command for RemoveAllGroups {
    const ID: u8 = 0x04;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl From<RemoveAllGroups> for crate::Cluster {
    fn from(command: RemoveAllGroups) -> Self {
        Self::Groups(command.into())
    }
}
