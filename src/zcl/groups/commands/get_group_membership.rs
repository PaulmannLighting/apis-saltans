use zb::types::{List, Uint16, Uint8};

use crate::zcl::groups::CLUSTER_ID;
use crate::{Cluster, Command};

/// Command to request the membership of a device in multiple groups.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GetGroupMembership {
    groups: List<Uint8, Uint16>,
}

impl GetGroupMembership {
    /// Creates a new `GetGroupMembership` command with the specified group count and list.
    #[must_use]
    pub const fn new(groups: List<Uint8, Uint16>) -> Self {
        Self { groups }
    }

    /// Return the groups the sender is a member of.
    #[must_use]
    pub fn groups(&self) -> &[Uint16] {
        self.groups.as_ref()
    }
}

impl AsRef<[Uint16]> for GetGroupMembership {
    fn as_ref(&self) -> &[Uint16] {
        self.groups()
    }
}

impl Cluster for GetGroupMembership {
    const ID: u16 = CLUSTER_ID;
}

impl Command for GetGroupMembership {
    const ID: u8 = 0x02;
}
