use le_stream::{FromLeStream, ToLeStream};
use zigbee::{Cluster, ClusterId, Direction};

use crate::{Command, Native};

/// Switch a device on.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct On;

impl Cluster for On {
    const ID: u16 = ClusterId::OnOff.as_u16();
}

impl Command for On {
    const ID: u8 = 0x01;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl Native for On {}

impl From<On> for crate::Cluster {
    fn from(command: On) -> Self {
        Self::OnOff(command.into())
    }
}
