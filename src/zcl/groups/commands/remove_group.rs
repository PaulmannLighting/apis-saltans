use le_stream::derive::{FromLeStream, ToLeStream};
use zigbee::types::Uint16;

use crate::zcl::groups::CLUSTER_ID;
use crate::{Cluster, Command};

/// Command to remove a group from the device's group table.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct RemoveGroup {
    group_id: Uint16,
}

impl RemoveGroup {
    /// Creates a new `RemoveGroup` command with the specified group ID.
    #[must_use]
    pub const fn new(group_id: Uint16) -> Self {
        Self { group_id }
    }

    /// Returns the identifier of the group to be removed.
    #[must_use]
    pub const fn group_id(self) -> Uint16 {
        self.group_id
    }
}

impl Cluster for RemoveGroup {
    const ID: u16 = CLUSTER_ID;
}

impl Command for RemoveGroup {
    const ID: u8 = 0x03;
}
