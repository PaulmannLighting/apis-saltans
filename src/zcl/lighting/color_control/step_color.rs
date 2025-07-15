use std::time::Duration;

use crate::zcl::Command;
use crate::zcl::constants::DECI_SECONDS_PER_MILLISECOND;
use crate::zcl::lighting::color_control::ColorControl;

/// Command to step a light's color.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StepColor {
    step_x: i16,
    step_y: i16,
    transition_time: u16,
}

impl StepColor {
    /// Create a new `StepColor` command.
    #[must_use]
    pub const fn new(step_x: i16, step_y: i16, transition_time: u16) -> Self {
        Self {
            step_x,
            step_y,
            transition_time,
        }
    }

    /// Return the step in the X color component.
    #[must_use]
    pub const fn step_x(self) -> i16 {
        self.step_x
    }

    /// Return the step in the Y color component.
    #[must_use]
    pub const fn step_y(self) -> i16 {
        self.step_y
    }

    /// Return the transition time.
    #[must_use]
    pub fn transition_time(self) -> Duration {
        Duration::from_millis(u64::from(self.transition_time) * DECI_SECONDS_PER_MILLISECOND)
    }
}

impl ColorControl for StepColor {}

impl Command for StepColor {
    const ID: u8 = 0x09;
}
