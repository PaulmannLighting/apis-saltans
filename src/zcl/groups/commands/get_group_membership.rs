use le_stream::derive::{FromLeStream, ToLeStream};
use le_stream::{ByteSizedVec, Prefixed};

use crate::zcl::groups::CLUSTER_ID;
use crate::zcl::{Cluster, Command};

/// Command to request the membership of a device in multiple groups.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct GetGroupMembership {
    group_list: Prefixed<u8, ByteSizedVec<u16>>,
}

impl GetGroupMembership {
    /// Creates a new `GetGroupMembership` command with the specified group count and list.
    #[must_use]
    pub const fn new(group_list: ByteSizedVec<u16>) -> Self {
        Self {
            group_list: Prefixed::new(group_list),
        }
    }

    /// Returns the number of groups in the membership request.
    #[must_use]
    pub fn group_count(&self) -> u8 {
        AsRef::<[u16]>::as_ref(&self.group_list).len() as u8
    }

    /// Returns the list of group IDs in the membership request.
    #[must_use]
    pub fn group_list(&self) -> &[u16] {
        self.group_list.as_ref()
    }
}

impl Cluster for GetGroupMembership {
    const ID: u16 = CLUSTER_ID;
}

impl Command for GetGroupMembership {
    const ID: u8 = 0x02;
}
