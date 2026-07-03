use apis_saltans_core::types::Uint8;
use apis_saltans_core::units::UnitsPerSecond;
use apis_saltans_core::{Cluster, ClusterId, Direction};
use le_stream::{FromLeStream, ToLeStream};

use crate::Command;
use crate::general::level::Mode;
use crate::options::Options;

/// Move with on/off command.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct MoveWithOnOff {
    mode: u8,
    rate: Uint8,
    options: Options,
}

impl MoveWithOnOff {
    /// Crate a new `MoveWithOnOff` command.
    #[must_use]
    pub fn new(mode: Mode<UnitsPerSecond>, options: Options) -> Self {
        Self {
            mode: mode.discriminant(),
            rate: mode.into_stride().into_inner(),
            options,
        }
    }

    /// Get the mode.
    #[must_use]
    pub fn mode(self) -> Option<Mode<UnitsPerSecond>> {
        Mode::new(self.mode, self.rate()?).ok()
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

impl Cluster<ClusterId> for MoveWithOnOff {
    const ID: ClusterId = ClusterId::Level;
}

impl Command for MoveWithOnOff {
    const ID: u8 = 0x05;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl From<MoveWithOnOff> for crate::Cluster {
    fn from(command: MoveWithOnOff) -> Self {
        Self::Level(command.into())
    }
}
