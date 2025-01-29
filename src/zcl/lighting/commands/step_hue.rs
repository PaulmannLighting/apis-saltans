use num_derive::FromPrimitive;

use crate::zcl::{Cluster, Command};

/// Command to step a light's hue.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StepHue {
    mode: Mode,
    size: u8,
    transition_time: u8,
}

impl StepHue {
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

impl Cluster for StepHue {
    const ID: u16 = 0x0300;
}

impl Command for StepHue {
    const ID: u8 = 0x02;
}

/// Mode of hue step.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Mode {
    Reserved1 = 0x00,
    Up = 0x01,
    Reserved2 = 0x02,
    Down = 0x03,
}
