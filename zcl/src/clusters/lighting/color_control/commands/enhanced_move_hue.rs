use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use zigbee::{Cluster, Direction};

use crate::Options;
use crate::clusters::lighting::color_control::CLUSTER_ID;
use crate::clusters::lighting::color_control::move_hue::Mode;
use crate::command::Command;

/// Command to move a light's hue in an enhanced way, allowing for more control over the rate.
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct EnhancedMoveHue {
    mode: u8,
    rate: u16,
    options: Options,
}

impl EnhancedMoveHue {
    /// Create a new `EnhancedMoveHue` command.
    #[must_use]
    pub const fn new(mode: Mode, rate: u16, options: Options) -> Self {
        Self {
            mode: mode as u8,
            rate,
            options,
        }
    }

    /// Return the mode of hue movement.
    ///
    /// # Errors
    ///
    /// Returns the raw mode value if it does not correspond to a valid `Mode` variant.
    pub fn mode(&self) -> Result<Mode, u8> {
        Mode::from_u8(self.mode).ok_or(self.mode)
    }

    /// Return the rate of hue change in steps per second.
    #[must_use]
    pub const fn rate(&self) -> u16 {
        self.rate
    }

    /// Return the options for the command.
    #[must_use]
    pub const fn options(&self) -> Options {
        self.options
    }
}

impl Cluster for EnhancedMoveHue {
    const ID: u16 = CLUSTER_ID;
}

impl Command for EnhancedMoveHue {
    const ID: u8 = 0x41;
    const DIRECTION: Direction = Direction::ClientToServer;
}
