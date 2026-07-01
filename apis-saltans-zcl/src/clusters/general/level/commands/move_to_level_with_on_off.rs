use le_stream::{FromLeStream, ToLeStream};
use apis_saltans_core::types::Uint16;
use apis_saltans_core::units::Deciseconds;
use apis_saltans_core::{ClusterId, Cluster, Direction};

use crate::Command;
use crate::options::Options;

/// Move to level with on/off command.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct MoveToLevelWithOnOff {
    level: u8,
    transition_time: Uint16,
    options: Options,
}

impl MoveToLevelWithOnOff {
    /// Creates a new `MoveToLevelWithOnOff` command.
    #[must_use]
    pub const fn new(level: u8, transition_time: Deciseconds, options: Options) -> Self {
        Self {
            level,
            transition_time: transition_time.into_inner(),
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
    pub fn transition_time(self) -> Option<Deciseconds> {
        self.transition_time.try_into().ok()
    }

    /// Get the options.
    #[must_use]
    pub const fn options(self) -> Options {
        self.options
    }
}

impl Cluster<ClusterId> for MoveToLevelWithOnOff {
    const ID: ClusterId = ClusterId::Level;
}

impl Command for MoveToLevelWithOnOff {
    const ID: u8 = 0x04;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl From<MoveToLevelWithOnOff> for crate::Cluster {
    fn from(command: MoveToLevelWithOnOff) -> Self {
        Self::Level(command.into())
    }
}
