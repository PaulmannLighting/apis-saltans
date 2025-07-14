use std::time::Duration;

use num_derive::FromPrimitive;

use crate::zcl::constants::DECI_SECONDS_PER_MILLISECOND;
use crate::zcl::{Cluster, Command};

/// Command to step a light to a specific hue.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StepSaturation {
    mode: Mode,
    size: u8,
    transition_time: u8,
}

impl StepSaturation {
    /// Create a new `StepSaturation` command.
    #[must_use]
    pub const fn new(mode: Mode, size: u8, transition_time: u8) -> Self {
        Self {
            mode,
            size,
            transition_time,
        }
    }

    /// Return the mode of saturation step.
    #[must_use]
    pub const fn mode(self) -> Mode {
        self.mode
    }

    /// Return the size of saturation step.
    #[must_use]
    pub const fn size(self) -> u8 {
        self.size
    }

    /// Return the transition time in deci-seconds.
    #[must_use]
    pub fn transition_time(self) -> Duration {
        Duration::from_millis(u64::from(self.transition_time) * DECI_SECONDS_PER_MILLISECOND)
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
