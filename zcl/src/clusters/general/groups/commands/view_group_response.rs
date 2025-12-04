use le_stream::{FromLeStream, ToLeStream};
use zigbee::Direction;
use zigbee::types::{String, Uint16};

use crate::clusters::general::groups::CLUSTER_ID;
use crate::{Cluster, Command, Status};

/// Represents a response to an `ViewGroup` command.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct ViewGroupResponse {
    status: u8,
    group_id: Uint16,
    group_name: String,
}

impl ViewGroupResponse {
    /// Creates a new `ViewGroupResponse` with the given status and group ID.
    #[must_use]
    pub fn new(status: Status, group_id: Uint16, group_name: String) -> Self {
        Self {
            status: status.into(),
            group_id,
            group_name,
        }
    }

    /// Returns the status of the response.
    ///
    /// # Errors
    ///
    /// If the status byte does not correspond to a valid `Status`, this will return the raw status value as an error.
    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    /// Returns the group ID associated with the response.
    #[must_use]
    pub const fn group_id(&self) -> Uint16 {
        self.group_id
    }

    /// Returns the name of the group associated with the response.
    #[must_use]
    pub const fn group_name(&self) -> &String {
        &self.group_name
    }
}

impl Cluster for ViewGroupResponse {
    const ID: u16 = CLUSTER_ID;
}

impl Command for ViewGroupResponse {
    const ID: u8 = 0x01;
    const DIRECTION: Direction = Direction::ServerToClient;
}
