use le_stream::{FromLeStream, ToLeStream};
use zigbee::{Cluster, Command, Direction};

use crate::clusters::general::groups::CLUSTER_ID;

/// Command to remove all groups from the device's group table.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct RemoveAllGroups;

impl Cluster for RemoveAllGroups {
    const ID: u16 = CLUSTER_ID;
}

impl Command for RemoveAllGroups {
    const ID: u8 = 0x04;
    const DIRECTION: Direction = Direction::ClientToServer;
}
