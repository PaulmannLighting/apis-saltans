use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use zigbee::{ClusterId, ClusterSpecific, Direction};

use crate::Command;
use crate::general::level::Mode;
use crate::options::Options;

/// Move with on/off command.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct MoveWithOnOff {
    mode: u8,
    rate: u8,
    options: Options,
}

impl MoveWithOnOff {
    /// Crate a new `MoveWithOnOff` command.
    #[must_use]
    pub const fn new(mode: Mode, rate: u8, options: Options) -> Self {
        Self {
            mode: mode as u8,
            rate,
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
    pub const fn rate(self) -> u8 {
        self.rate
    }

    /// Get the options.
    #[must_use]
    pub const fn options(self) -> Options {
        self.options
    }
}

impl ClusterSpecific for MoveWithOnOff {
    const CLUSTER: ClusterId = ClusterId::Level;
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
