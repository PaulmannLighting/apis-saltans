use le_stream::{FromLeStream, ToLeStream};
use zigbee::{Cluster, ClusterId, Direction};

use crate::{Command, Native};

/// Reset a device to factory defaults.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
pub struct ResetToFactoryDefaults;

impl Cluster for ResetToFactoryDefaults {
    const ID: u16 = ClusterId::Basic.as_u16();
}

impl Command for ResetToFactoryDefaults {
    const ID: u8 = 0x00;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl Native for ResetToFactoryDefaults {}

impl From<ResetToFactoryDefaults> for crate::Cluster {
    fn from(command: ResetToFactoryDefaults) -> Self {
        Self::Basic(command.into())
    }
}
