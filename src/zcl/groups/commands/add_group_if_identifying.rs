use core::str::Utf8Error;

use crate::types::String;
use crate::zcl::groups::CLUSTER_ID;
use crate::zcl::{Cluster, Command};

/// Command to add a group to the device's group table if the device is currently identifying.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AddGroupIfIdentifying {
    group_id: u16,
    group_name: String,
}

impl AddGroupIfIdentifying {
    /// Creates a new `AddGroupIfIdentifying` command with the specified group ID and name.
    #[must_use]
    pub const fn new(group_id: u16, group_name: String) -> Self {
        Self {
            group_id,
            group_name,
        }
    }

    /// Returns the identifier of the group to be added.
    #[must_use]
    pub fn group_id(&self) -> u16 {
        self.group_id
    }

    /// Returns the name of the group to be added.
    pub fn group_name(&self) -> Result<&str, Utf8Error> {
        self.group_name.try_as_str()
    }

    /// Returns the raw bytes of the group name.
    #[must_use]
    pub fn group_name_raw(&self) -> &[u8] {
        self.group_name.as_ref()
    }
}

impl Cluster for AddGroupIfIdentifying {
    const ID: u16 = CLUSTER_ID;
}

impl Command for AddGroupIfIdentifying {
    const ID: u8 = 0x05;
}
