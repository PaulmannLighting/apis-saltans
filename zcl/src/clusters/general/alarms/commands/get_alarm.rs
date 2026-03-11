use le_stream::{FromLeStream, ToLeStream};
use zigbee::{Cluster, Direction};

use super::CLUSTER_ID;
use crate::Command;

/// Returns the alarm with the earliest generated entry.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, FromLeStream, ToLeStream,
)]
pub struct GetAlarm;

impl Cluster for GetAlarm {
    const ID: u16 = CLUSTER_ID;
}

impl Command for GetAlarm {
    const ID: u8 = 0x02;
    const DIRECTION: Direction = Direction::ClientToServer;
}
