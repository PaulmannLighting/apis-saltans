use le_stream::{FromLeStream, ToLeStream};
use apis_saltans_core::types::{String, Uint16};
use apis_saltans_core::{ClusterId, Cluster, Direction};

use crate::Command;

/// Command to add a group to the device's group table if the device is currently identifying.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct AddGroupIfIdentifying {
    group_id: Uint16,
    group_name: String,
}

impl AddGroupIfIdentifying {
    /// Creates a new `AddGroupIfIdentifying` command with the specified group ID and name.
    #[must_use]
    pub const fn new(group_id: Uint16, group_name: String) -> Self {
        Self {
            group_id,
            group_name,
        }
    }

    /// Returns the identifier of the group to be added.
    #[must_use]
    pub const fn group_id(&self) -> Uint16 {
        self.group_id
    }

    /// Returns the name of the group to be added.
    #[must_use]
    pub const fn group_name(&self) -> &String {
        &self.group_name
    }
}

impl Cluster<ClusterId> for AddGroupIfIdentifying {
    const ID: ClusterId = ClusterId::Groups;
}

impl Command for AddGroupIfIdentifying {
    const ID: u8 = 0x05;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl From<AddGroupIfIdentifying> for crate::Cluster {
    fn from(command: AddGroupIfIdentifying) -> Self {
        Self::Groups(command.into())
    }
}
