use core::time::Duration;

use zigbee::{Cluster, Direction, FromDeciSeconds};

use crate::Command;
use crate::clusters::lighting::color_control::CLUSTER_ID;

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
        Duration::from_deci_seconds(self.transition_time)
    }
}

impl Cluster for StepColor {
    const ID: u16 = CLUSTER_ID;
}

impl Command for StepColor {
    const ID: u8 = 0x09;
    const DIRECTION: Direction = Direction::ClientToServer;
}
