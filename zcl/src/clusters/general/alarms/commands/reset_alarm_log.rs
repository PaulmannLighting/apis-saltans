use le_stream::{FromLeStream, ToLeStream};
use zigbee::{Cluster, Direction};

use super::CLUSTER_ID;
use crate::Command;

/// Clear the alarm table.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, FromLeStream, ToLeStream,
)]
pub struct ResetAlarmLog;

impl Cluster for ResetAlarmLog {
    const ID: u16 = CLUSTER_ID;
}

impl Command for ResetAlarmLog {
    const ID: u8 = 0x03;
    const DIRECTION: Direction = Direction::ClientToServer;
}
