use le_stream::{FromLeStream, ToLeStream};
use zigbee::{ClusterId, ClusterSpecific, Direction};

use crate::Command;
use crate::options::Options;

/// Stop command.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
pub struct StopWithOnOff {
    options: Options,
}

impl StopWithOnOff {
    /// Creates a new `StopWithOnOff` command.
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

impl ClusterSpecific for StopWithOnOff {
    const CLUSTER: ClusterId = ClusterId::Level;
}

impl Command for StopWithOnOff {
    const ID: u8 = 0x07;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl From<StopWithOnOff> for crate::Cluster {
    fn from(command: StopWithOnOff) -> Self {
        Self::Level(command.into())
    }
}
