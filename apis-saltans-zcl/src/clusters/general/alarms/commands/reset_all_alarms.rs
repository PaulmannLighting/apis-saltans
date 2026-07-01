use apis_saltans_core::{Cluster, ClusterId, Direction};
use le_stream::{FromLeStream, ToLeStream};

use crate::Command;

/// Reset all alarms.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, FromLeStream, ToLeStream,
)]
pub struct ResetAllAlarms;

impl Cluster<ClusterId> for ResetAllAlarms {
    const ID: ClusterId = ClusterId::Alarms;
}

impl Command for ResetAllAlarms {
    const ID: u8 = 0x01;
    const DIRECTION: Direction = Direction::ClientToServer;
}
