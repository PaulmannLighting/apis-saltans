use num_derive::FromPrimitive;

use crate::zcl::{Cluster, Command};

/// Command to step a light to a specific hue.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StepSaturation {
    mode: Mode,
    size: u8,
    transition_time: u8,
}

impl StepSaturation {
    #[must_use]
    pub const fn new(mode: Mode, size: u8, transition_time: u8) -> Self {
        Self {
            mode,
            size,
            transition_time,
        }
    }

    #[must_use]
    pub const fn mode(self) -> Mode {
        self.mode
    }

    #[must_use]
    pub const fn size(self) -> u8 {
        self.size
    }

    #[must_use]
    pub const fn transition_time(self) -> u8 {
        self.transition_time
    }
}

impl Cluster for StepSaturation {
    const ID: u16 = 0x0300;
}

impl Command for StepSaturation {
    const ID: u8 = 0x04;
}

/// Step mode.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Mode {
    // 0x00 is reserved.
    /// Step up.
    Up = 0x01,
    // 0x02 is reserved.
    /// Step down.
    Down = 0x03,
}
