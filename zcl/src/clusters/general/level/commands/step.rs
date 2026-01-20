use le_stream::{FromLeStream, ToLeStream};
use zigbee::{Cluster, Direction};

use crate::Command;
use crate::general::level::CLUSTER_ID;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct Step {
    mode: u8,
    size: u8,
    transition_time: u16,
    options_mask: Option<u8>,
    options_override: Option<u8>,
}

impl Cluster for Step {
    const ID: u16 = CLUSTER_ID;
}

impl Command for Step {
    const ID: u8 = 0x02;
    const DIRECTION: Direction = Direction::ClientToServer;
}
