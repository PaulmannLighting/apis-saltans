use le_stream::{FromLeStream, ToLeStream};
use zigbee::{Cluster, Direction};

use crate::Command;
use crate::general::on_off::CLUSTER_ID;

/// Switch a device on and recall its settings of before it was switched off.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct OnWithRecallGlobalScene;

impl Cluster for OnWithRecallGlobalScene {
    const ID: u16 = CLUSTER_ID;
}

impl Command for OnWithRecallGlobalScene {
    const ID: u8 = 0x41;
    const DIRECTION: Direction = Direction::ClientToServer;
}
