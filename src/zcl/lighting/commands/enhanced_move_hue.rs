use crate::zcl::Command;
use crate::zcl::lighting::Lighting;
use crate::zcl::lighting::move_hue::Mode;

/// Command to move a light's hue in an enhanced way, allowing for more control over the rate.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct EnhancedMoveHue {
    mode: Mode,
    rate: u16,
}

impl EnhancedMoveHue {
    /// Create a new `EnhancedMoveHue` command.
    #[must_use]
    pub const fn new(mode: Mode, rate: u16) -> Self {
        Self { mode, rate }
    }

    /// Return the misc of hue movement.
    #[must_use]
    pub const fn mode(self) -> Mode {
        self.mode
    }

    /// Return the rate of hue change in steps per second.
    #[must_use]
    pub const fn rate(self) -> u16 {
        self.rate
    }
}

impl Lighting for EnhancedMoveHue {}

impl Command for EnhancedMoveHue {
    const ID: u8 = 0x41;
}
