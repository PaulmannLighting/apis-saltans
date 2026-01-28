use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use zigbee::{Cluster, Direction};

use super::CLUSTER_ID;
use crate::Command;
use crate::general::level::Mode;

/// Move with on/off command.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct MoveWithOnOff {
    mode: u8,
    rate: u8,
    options_mask: Option<u8>,
    options_override: Option<u8>,
}

impl MoveWithOnOff {
    /// Crate a new `MoveWithOnOff` command.
    #[must_use]
    pub const fn new(mode: Mode, rate: u8, options_mask: u8, options_override: u8) -> Self {
        Self {
            mode: mode as u8,
            rate,
            options_mask: Some(options_mask),
            options_override: Some(options_override),
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
    pub fn options_mask(self) -> u8 {
        self.options_mask.unwrap_or_default()
    }

    /// Get the options override.
    #[must_use]
    pub fn options_override(self) -> u8 {
        self.options_override.unwrap_or_default()
    }
}

impl Cluster for MoveWithOnOff {
    const ID: u16 = CLUSTER_ID;
}

impl Command for MoveWithOnOff {
    const ID: u8 = 0x05;
    const DIRECTION: Direction = Direction::ClientToServer;
}
