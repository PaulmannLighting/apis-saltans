use le_stream::{FromLeStream, ToLeStream};
use apis_saltans_core::{ClusterId, ClusterSpecific, Direction};

use crate::Command;

/// Returns the alarm with the earliest generated entry.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, FromLeStream, ToLeStream,
)]
pub struct GetAlarm;

impl ClusterSpecific for GetAlarm {
    const CLUSTER: ClusterId = ClusterId::Alarms;
}

impl Command for GetAlarm {
    const ID: u8 = 0x02;
    const DIRECTION: Direction = Direction::ClientToServer;
}
