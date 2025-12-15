use le_stream::{FromLeStream, ToLeStream};
use zigbee::{Cluster, Direction};

use crate::Command;
use crate::clusters::general::on_off::CLUSTER_ID;

/// Switch a device on.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct On;

impl Cluster for On {
    const ID: u16 = CLUSTER_ID;
}

impl Command for On {
    const ID: u8 = 0x01;
    const DIRECTION: Direction = Direction::ClientToServer;
}
