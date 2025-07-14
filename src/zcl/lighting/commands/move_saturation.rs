use crate::zcl::{Command, lighting::Lighting, lighting::mode::move_saturation::Mode};

/// Command to move a light's saturation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MoveSaturation {
    mode: Mode,
    /// Steps per second.
    rate: u8,
}

impl MoveSaturation {
    /// Create a new `MoveSaturation` command.
    #[must_use]
    pub const fn new(mode: Mode, rate: u8) -> Self {
        Self { mode, rate }
    }

    /// Return the mode.
    #[must_use]
    pub const fn mode(self) -> Mode {
        self.mode
    }

    /// Return the rate of saturation change in steps per second.
    #[must_use]
    pub const fn rate(self) -> u8 {
        self.rate
    }
}

impl Lighting for MoveSaturation {}

impl Command for MoveSaturation {
    const ID: u8 = 0x04;
}
