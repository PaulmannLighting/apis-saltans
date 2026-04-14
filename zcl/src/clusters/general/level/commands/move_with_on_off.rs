use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use zigbee::{Cluster, Direction};

use super::CLUSTER_ID;
use crate::general::level::Mode;
use crate::options::Options;
use crate::{Command, Native};

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

impl Cluster for MoveWithOnOff {
    const ID: u16 = CLUSTER_ID;
}

impl Command for MoveWithOnOff {
    const ID: u8 = 0x05;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl Native for MoveWithOnOff {}

#[cfg(feature = "smarthomelib")]
mod smarthomelib {
    use smarthomelib::Dimming;

    use super::MoveWithOnOff;
    use crate::general::level::Mode;

    impl TryFrom<MoveWithOnOff> for Dimming {
        type Error = u8;

        fn try_from(value: MoveWithOnOff) -> Result<Self, Self::Error> {
            match value.mode()? {
                Mode::Up => Ok(Self::Up { rate: value.rate() }),
                Mode::Down => Ok(Self::Down { rate: value.rate() }),
            }
        }
    }
}
