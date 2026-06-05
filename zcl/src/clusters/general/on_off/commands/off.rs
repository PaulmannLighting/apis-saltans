use le_stream::{FromLeStream, ToLeStream};
use zigbee::{Cluster, ClusterId, Direction};

use crate::{Command, Native};

/// Switch a device off.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct Off;

impl Cluster for Off {
    const ID: u16 = ClusterId::OnOff.as_u16();
}

impl Command for Off {
    const ID: u8 = 0x00;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl Native for Off {}

impl From<Off> for crate::Cluster {
    fn from(command: Off) -> Self {
        Self::OnOff(command.into())
    }
}
