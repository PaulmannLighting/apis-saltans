use core::time::Duration;

use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use zigbee::{Cluster, Direction, FromDeciSeconds};

use super::CLUSTER_ID;
use crate::Command;
use crate::general::level::Mode;

/// Step command.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct Step {
    mode: u8,
    size: u8,
    transition_time: u16,
    options_mask: u8,
    options_override: u8,
}

impl Step {
    /// Creates a new `Step` command.
    #[must_use]
    pub const fn new(
        mode: Mode,
        size: u8,
        transition_time: u16,
        options_mask: u8,
        options_override: u8,
    ) -> Self {
        Self {
            mode: mode as u8,
            size,
            transition_time,
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

    /// Get the size.
    #[must_use]
    pub const fn size(self) -> u8 {
        self.size
    }

    /// Get the transition time.
    #[must_use]
    pub fn transition_time(self) -> Option<Duration> {
        Option::<u16>::from(self.transition_time).map(Duration::from_deci_seconds)
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

impl Cluster for Step {
    const ID: u16 = CLUSTER_ID;
}

impl Command for Step {
    const ID: u8 = 0x02;
    const DIRECTION: Direction = Direction::ClientToServer;
}
