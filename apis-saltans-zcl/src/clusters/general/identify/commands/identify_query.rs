use apis_saltans_core::{Cluster, ClusterId, Direction};
use le_stream::{FromLeStream, ToLeStream};

use crate::Command;

/// Request the target to respond if they are currently identifying themselves.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct IdentifyQuery;

impl Cluster<ClusterId> for IdentifyQuery {
    const ID: ClusterId = ClusterId::Identify;
}

impl Command for IdentifyQuery {
    const ID: u8 = 0x01;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl From<IdentifyQuery> for crate::Cluster {
    fn from(command: IdentifyQuery) -> Self {
        Self::Identify(command.into())
    }
}
