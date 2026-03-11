use le_stream::{FromLeStream, ToLeStream};
use zigbee::{Cluster, Direction};

use super::CLUSTER_ID;
use crate::Command;

/// Reset all alarms.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, FromLeStream, ToLeStream,
)]
pub struct ResetAllAlarms;

impl Cluster for ResetAllAlarms {
    const ID: u16 = CLUSTER_ID;
}

impl Command for ResetAllAlarms {
    const ID: u8 = 0x01;
    const DIRECTION: Direction = Direction::ClientToServer;
}
