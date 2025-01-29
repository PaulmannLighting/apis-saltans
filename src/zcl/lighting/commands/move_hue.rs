use num_derive::FromPrimitive;

use crate::zcl::{Cluster, Command};

/// Command to move a light's hue.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MoveHue {
    mode: Mode,
    /// Steps per second.
    rate: u8,
}

impl MoveHue {
    #[must_use]
    pub const fn new(mode: Mode, rate: u8) -> Self {
        Self { mode, rate }
    }

    #[must_use]
    pub const fn mode(self) -> Mode {
        self.mode
    }

    #[must_use]
    pub const fn rate(self) -> u8 {
        self.rate
    }
}

impl Cluster for MoveHue {
    const ID: u16 = 0x0300;
}

impl Command for MoveHue {
    const ID: u8 = 0x01;
}

/// Move mode.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Mode {
    Stop = 0x00,
    Up = 0x01,
    // 0x02 is reserved.
    Down = 0x03,
}
