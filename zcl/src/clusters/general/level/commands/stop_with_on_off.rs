use le_stream::{FromLeStream, ToLeStream};
use zigbee::{Cluster, Direction};

use super::CLUSTER_ID;
use crate::Command;

/// Stop command.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct StopWithOnOff {
    options_mask: u8,
    options_override: u8,
}

impl Cluster for StopWithOnOff {
    const ID: u16 = CLUSTER_ID;
}

impl Command for StopWithOnOff {
    const ID: u8 = 0x07;
    const DIRECTION: Direction = Direction::ClientToServer;
}
