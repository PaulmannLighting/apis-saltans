use core::time::Duration;

use le_stream::{FromLeStream, ToLeStream};
use zigbee::types::Uint16;
use zigbee::{Cluster, Direction, FromDeciSeconds};

use crate::options::Options;
use crate::{ClusterId, Command, Native};

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

    /// Get the transition time.
    #[must_use]
    pub fn transition_time(self) -> Option<Duration> {
        Option::<u16>::from(self.transition_time).map(Duration::from_deci_seconds)
    }

    /// Get the options.
    #[must_use]
    pub const fn options(self) -> Options {
        self.options
    }
}

impl Cluster for MoveToLevel {
    const ID: u16 = ClusterId::Level.as_u16();
}

impl Command for MoveToLevel {
    const ID: u8 = 0x00;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl Native for MoveToLevel {}

impl From<MoveToLevel> for crate::Cluster {
    fn from(command: MoveToLevel) -> Self {
        Self::Level(command.into())
    }
}
