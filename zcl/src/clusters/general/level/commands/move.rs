use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use zigbee::{Cluster, Direction};

use super::CLUSTER_ID;
use crate::Command;
use crate::general::level::Mode;

/// Move command.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct Move {
    mode: u8,
    rate: u8,
    options_mask: u8,
    options_override: u8,
}

impl Move {
    /// Crate a new `Move` command.
    #[must_use]
    pub const fn new(mode: Mode, rate: u8, options_mask: u8, options_override: u8) -> Self {
        Self {
            mode: mode as u8,
            rate,
            options_mask,
            options_override,
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

    /// Get the options mask.
    #[must_use]
    pub const fn options_mask(self) -> u8 {
        self.options_mask
    }

    /// Get the options override.
    #[must_use]
    pub const fn options_override(self) -> u8 {
        self.options_override
    }
}

impl Cluster for Move {
    const ID: u16 = CLUSTER_ID;
}

impl Command for Move {
    const ID: u8 = 0x01;
    const DIRECTION: Direction = Direction::ClientToServer;
}
