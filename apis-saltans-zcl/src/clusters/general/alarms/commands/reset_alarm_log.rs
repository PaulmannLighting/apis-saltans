use le_stream::{FromLeStream, ToLeStream};
use apis_saltans_core::{ClusterId, Cluster, Direction};

use crate::Command;

/// Clear the alarm table.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, FromLeStream, ToLeStream,
)]
pub struct ResetAlarmLog;

impl Cluster<ClusterId> for ResetAlarmLog {
    const ID: ClusterId = ClusterId::Alarms;
}

impl Command for ResetAlarmLog {
    const ID: u8 = 0x03;
    const DIRECTION: Direction = Direction::ClientToServer;
}
