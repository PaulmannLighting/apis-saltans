use apis_saltans_core::types::Uint8;
use apis_saltans_core::units::UnitsPerSecond;
use apis_saltans_core::{Cluster, ClusterId, Direction};
use le_stream::{FromLeStream, ToLeStream};

use crate::Command;
use crate::general::level::Mode;
use crate::options::Options;

/// Move command.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct Move {
    mode: u8,
    rate: Uint8,
    options: Options,
}

impl Move {
    /// Crate a new `Move` command.
    #[must_use]
    pub fn new(mode: Mode<UnitsPerSecond>, options: Options) -> Self {
        Self {
            mode: mode.discriminant(),
            rate: mode.into_stride().into_inner(),
            options,
        }
    }

    /// Get the mode.
    pub fn mode(self) -> Option<Mode<UnitsPerSecond>> {
        Mode::new(self.mode, self.rate()?).ok()
    }

    /// Get the rate.
    pub fn rate(self) -> Option<UnitsPerSecond> {
        self.rate.try_into().ok()
    }

    /// Get the options.
    #[must_use]
    pub const fn options(self) -> Options {
        self.options
    }
}

impl Cluster<ClusterId> for Move {
    const ID: ClusterId = ClusterId::Level;
}

impl Command for Move {
    const ID: u8 = 0x01;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl From<Move> for crate::Cluster {
    fn from(command: Move) -> Self {
        Self::Level(command.into())
    }
}
