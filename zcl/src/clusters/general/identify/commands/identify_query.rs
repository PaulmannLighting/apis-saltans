use le_stream::{FromLeStream, ToLeStream};
use zigbee::{Cluster, Direction};

use crate::Command;
use crate::clusters::general::identify::CLUSTER_ID;

/// Request the target to respond if they are currently identifying themselves.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct IdentifyQuery;

impl Cluster for IdentifyQuery {
    const ID: u16 = CLUSTER_ID;
}

impl Command for IdentifyQuery {
    const ID: u8 = 0x01;
    const DIRECTION: Direction = Direction::ClientToServer;
}
