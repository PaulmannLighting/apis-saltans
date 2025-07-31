use core::str::Utf8Error;

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::types::{String, Uint16};
use crate::zcl::groups::CLUSTER_ID;
use crate::zcl::{Cluster, Command};

/// Command to add a group to the device's group table.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct AddGroup {
    /// The identifier of the group to be added.
    group_id: Uint16,
    /// The name of the group to be added, if supported.
    group_name: String,
}

impl AddGroup {
    /// Creates a new `AddGroup` command with the specified group ID and name.
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
    ///
    /// # Errors
    ///
    /// If the group name is not valid UTF-8, this will return an [`Utf8Error`].
    pub fn group_name(&self) -> Result<&str, Utf8Error> {
        self.group_name.try_as_str()
    }

    /// Returns the raw bytes of the group name.
    #[must_use]
    pub fn group_name_raw(&self) -> &[u8] {
        self.group_name.as_ref()
    }
}

impl Cluster for AddGroup {
    const ID: u16 = CLUSTER_ID;
}

impl Command for AddGroup {
    const ID: u8 = 0x00;
}
