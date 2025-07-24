use le_stream::derive::{FromLeStream, ToLeStream};

use crate::zcl::groups::CLUSTER_ID;
use crate::zcl::{Cluster, Command};

/// Command to remove all groups from the device's group table.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct RemoveAllGroups;

impl Cluster for RemoveAllGroups {
    const ID: u16 = CLUSTER_ID;
}

impl Command for RemoveAllGroups {
    const ID: u8 = 0x04;
}
