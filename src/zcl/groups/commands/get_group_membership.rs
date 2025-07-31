use crate::types::{List, Uint8, Uint16};
use crate::zcl::groups::CLUSTER_ID;
use crate::zcl::{Cluster, Command};

/// Command to request the membership of a device in multiple groups.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GetGroupMembership {
    group_list: List<Uint8, Uint16>,
}

impl GetGroupMembership {
    /// Creates a new `GetGroupMembership` command with the specified group count and list.
    #[must_use]
    pub const fn new(group_list: List<Uint8, Uint16>) -> Self {
        Self { group_list }
    }
}

impl AsRef<[Uint16]> for GetGroupMembership {
    fn as_ref(&self) -> &[Uint16] {
        self.group_list.as_ref()
    }
}

impl Cluster for GetGroupMembership {
    const ID: u16 = CLUSTER_ID;
}

impl Command for GetGroupMembership {
    const ID: u8 = 0x02;
}
