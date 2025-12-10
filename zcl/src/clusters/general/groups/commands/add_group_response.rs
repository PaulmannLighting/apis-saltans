use le_stream::{FromLeStream, ToLeStream};
use zigbee::types::Uint16;
use zigbee::{Cluster, Command, Direction};

use crate::Status;
use crate::clusters::general::groups::CLUSTER_ID;

/// Represents a response to an `AddGroups` command.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct AddGroupResponse {
    status: u8,
    group_id: Uint16,
}

impl AddGroupResponse {
    /// Creates a new `AddGroupsResponse` with the given status and group ID.
    #[must_use]
    pub fn new(status: Status, group_id: Uint16) -> Self {
        Self {
            status: status.into(),
            group_id,
        }
    }

    /// Returns the status of the response.
    ///
    /// # Errors
    ///
    /// If the status byte does not correspond to a valid `Status`, this will return the raw status value as an error.
    pub fn status(self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    /// Returns the group ID associated with the response.
    #[must_use]
    pub const fn group_id(self) -> Uint16 {
        self.group_id
    }
}

impl Cluster for AddGroupResponse {
    const ID: u16 = CLUSTER_ID;
}

impl Command for AddGroupResponse {
    const ID: u8 = 0x00;
    const DIRECTION: Direction = Direction::ServerToClient;
}
