use le_stream::derive::{FromLeStream, ToLeStream};
use zigbee::types::{List, Uint8, Uint16};

use crate::clusters::general::groups::CLUSTER_ID;
use crate::{Cluster, Command};

/// Represents a response to an `GetGroupMembership` command.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct GetGroupMembershipResponse {
    capacity: Uint8,
    groups: List<Uint8, Uint16>,
}

impl GetGroupMembershipResponse {
    /// Creates a new `GetGroupMembershipResponse` with the given status and group ID.
    #[must_use]
    pub const fn new(capacity: Uint8, groups: List<Uint8, Uint16>) -> Self {
        Self { capacity, groups }
    }

    /// Return the remaining capacity of the group table.
    #[must_use]
    pub const fn capacity(&self) -> Uint8 {
        self.capacity
    }

    /// Return the groups in the group table.
    #[must_use]
    pub fn groups(&self) -> &[Uint16] {
        self.groups.as_ref()
    }
}

impl AsRef<[Uint16]> for GetGroupMembershipResponse {
    fn as_ref(&self) -> &[Uint16] {
        self.groups()
    }
}

impl Cluster for GetGroupMembershipResponse {
    const ID: u16 = CLUSTER_ID;
}

impl Command for GetGroupMembershipResponse {
    const ID: u8 = 0x02;
}
