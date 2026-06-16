use le_stream::{FromLeStream, ToLeStream};
use zigbee::{ClusterId, ClusterSpecific, Direction};

use crate::Command;

/// Reset a device to factory defaults.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
pub struct ResetToFactoryDefaults;

impl ClusterSpecific for ResetToFactoryDefaults {
    const CLUSTER: ClusterId = ClusterId::Basic;
}

impl Command for ResetToFactoryDefaults {
    const ID: u8 = 0x00;
    const DIRECTION: Direction = Direction::ClientToServer;
}


impl From<ResetToFactoryDefaults> for crate::Cluster {
    fn from(command: ResetToFactoryDefaults) -> Self {
        Self::Basic(command.into())
    }
}
