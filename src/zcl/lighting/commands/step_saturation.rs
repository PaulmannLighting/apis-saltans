use std::time::Duration;

use crate::zcl::{
    Command, constants::DECI_SECONDS_PER_MILLISECOND, lighting::Lighting,
    lighting::step_saturation::Mode,
};

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

impl Lighting for StepSaturation {}

impl Command for StepSaturation {
    const ID: u8 = 0x04;
}
