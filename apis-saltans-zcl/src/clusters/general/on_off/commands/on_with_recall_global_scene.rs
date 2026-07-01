use apis_saltans_core::{Cluster, ClusterId, Direction};
use le_stream::{FromLeStream, ToLeStream};

use crate::Command;

/// Switch a device on and recall its settings of before it was switched off.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct OnWithRecallGlobalScene;

impl Cluster<ClusterId> for OnWithRecallGlobalScene {
    const ID: ClusterId = ClusterId::OnOff;
}

impl Command for OnWithRecallGlobalScene {
    const ID: u8 = 0x41;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl From<OnWithRecallGlobalScene> for crate::Cluster {
    fn from(command: OnWithRecallGlobalScene) -> Self {
        Self::OnOff(command.into())
    }
}
