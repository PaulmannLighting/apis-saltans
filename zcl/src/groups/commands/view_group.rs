use le_stream::derive::{FromLeStream, ToLeStream};
use zigbee::types::Uint16;

use crate::groups::CLUSTER_ID;
use crate::{Cluster, Command};

/// Command to view a group in the device's group table.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct ViewGroup {
    /// The identifier of the group to view.
    group_id: Uint16,
}

impl ViewGroup {
    /// Creates a new `ViewGroup` command with the specified group ID.
    #[must_use]
    pub const fn new(group_id: Uint16) -> Self {
        Self { group_id }
    }

    /// Returns the identifier of the group to view.
    #[must_use]
    pub const fn group_id(self) -> Uint16 {
        self.group_id
    }
}

impl Cluster for ViewGroup {
    const ID: u16 = CLUSTER_ID;
}

impl Command for ViewGroup {
    const ID: u8 = 0x01;
}
