use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use zigbee::types::Uint8;
use zigbee::units::UnitsPerSecond;
use zigbee::{ClusterId, ClusterSpecific, Direction};

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
    pub const fn new(mode: Mode, rate: UnitsPerSecond, options: Options) -> Self {
        Self {
            mode: mode as u8,
            rate: rate.into_inner(),
            options,
        }
    }

    /// Get the mode.
    ///
    /// # Errors
    ///
    /// Returns the raw mode value if it is invalid.
    pub fn mode(self) -> Result<Mode, u8> {
        Mode::from_u8(self.mode).ok_or(self.mode)
    }

    /// Get the rate.
    #[must_use]
    pub fn rate(self) -> Option<UnitsPerSecond> {
        self.rate.try_into().ok()
    }

    /// Get the options.
    #[must_use]
    pub const fn options(self) -> Options {
        self.options
    }
}

impl ClusterSpecific for Move {
    const CLUSTER: ClusterId = ClusterId::Level;
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
