use le_stream::{FromLeStream, ToLeStream};
use apis_saltans_core::{ClusterId, ClusterSpecific, Direction};

use crate::Command;
use crate::options::Options;

/// Stop command.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
pub struct Stop {
    options: Options,
}

impl Stop {
    /// Creates a new `Stop` command.
    #[must_use]
    pub const fn new(options: Options) -> Self {
        Self { options }
    }

    /// Get the options.
    #[must_use]
    pub const fn options(self) -> Options {
        self.options
    }
}

impl ClusterSpecific for Stop {
    const CLUSTER: ClusterId = ClusterId::Level;
}

impl Command for Stop {
    const ID: u8 = 0x03;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl From<Stop> for crate::Cluster {
    fn from(command: Stop) -> Self {
        Self::Level(command.into())
    }
}
