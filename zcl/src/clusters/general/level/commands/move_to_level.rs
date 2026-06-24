use le_stream::{FromLeStream, ToLeStream};
use zigbee::types::Uint16;
use zigbee::{ClusterId, ClusterSpecific, Direction};

use crate::Command;
use crate::options::Options;

/// Move to level command.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct MoveToLevel {
    level: u8,
    transition_time: Uint16,
    options: Options,
}

impl MoveToLevel {
    /// Creates a new `MoveToLevel` command.
    #[must_use]
    pub const fn new(level: u8, transition_time: Uint16, options: Options) -> Self {
        Self {
            level,
            transition_time,
            options,
        }
    }

    /// Get the level.
    #[must_use]
    pub const fn level(self) -> u8 {
        self.level
    }

    /// Return the transition time, if any, in deciseconds.
    #[must_use]
    pub fn transition_time(self) -> Option<u16> {
        self.transition_time.into()
    }

    /// Get the options.
    #[must_use]
    pub const fn options(self) -> Options {
        self.options
    }
}

impl ClusterSpecific for MoveToLevel {
    const CLUSTER: ClusterId = ClusterId::Level;
}

impl Command for MoveToLevel {
    const ID: u8 = 0x00;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl From<MoveToLevel> for crate::Cluster {
    fn from(command: MoveToLevel) -> Self {
        Self::Level(command.into())
    }
}
