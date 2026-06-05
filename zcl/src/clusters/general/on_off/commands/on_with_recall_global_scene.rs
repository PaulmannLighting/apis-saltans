use le_stream::{FromLeStream, ToLeStream};
use zigbee::{Cluster, Direction};

use crate::{ClusterId, Command, Native};

/// Switch a device on and recall its settings of before it was switched off.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct OnWithRecallGlobalScene;

impl Cluster for OnWithRecallGlobalScene {
    const ID: u16 = ClusterId::OnOff.as_u16();
}

impl Command for OnWithRecallGlobalScene {
    const ID: u8 = 0x41;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl Native for OnWithRecallGlobalScene {}

impl From<OnWithRecallGlobalScene> for crate::Cluster {
    fn from(command: OnWithRecallGlobalScene) -> Self {
        Self::OnOff(command.into())
    }
}
