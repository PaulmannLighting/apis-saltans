use num_derive::FromPrimitive;

use crate::zcl::{Cluster, Command};

/// Command to move a light's saturation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MoveSaturation {
    mode: Mode,
    /// Steps per second.
    rate: u8,
}

impl MoveSaturation {
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

impl Cluster for MoveSaturation {
    const ID: u16 = 0x0300;
}

impl Command for MoveSaturation {
    const ID: u8 = 0x04;
}

/// Mode of saturation move.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Mode {
    /// Stop move.
    Stop = 0x00,
    /// Move up.
    Up = 0x01,
    // 0x02 is reserved.
    /// Move down.
    Down = 0x03,
}
