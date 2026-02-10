use core::num::TryFromIntError;
use core::time::Duration;

use zigbee::{Cluster, Direction, FromDeciSeconds, IntoDeciSeconds};

use crate::clusters::lighting::color_control::CLUSTER_ID;
use crate::{Command, Options};

/// Command to step a light's color.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct StepColor {
    step_x: i16,
    step_y: i16,
    transition_time: u16,
    options: Options,
}

impl StepColor {
    /// Create a new `StepColor` command.
    #[must_use]
    pub const fn new(step_x: i16, step_y: i16, transition_time: u16, options: Options) -> Self {
        Self {
            step_x,
            step_y,
            transition_time,
            options,
        }
    }

    /// Try to create a new `StepColor` command.
    ///
    /// # Errors
    ///
    /// Returns an [`TryFromIntError`] if the resulting deci-seconds value cannot fit in a `u16`.
    pub fn try_new(
        step_x: i16,
        step_y: i16,
        transition_time: Duration,
        options: Options,
    ) -> Result<Self, TryFromIntError> {
        transition_time
            .into_deci_seconds()
            .try_into()
            .map(|transition_time| Self::new(step_x, step_y, transition_time, options))
    }

    /// Return the step in the X color component.
    #[must_use]
    pub const fn step_x(&self) -> i16 {
        self.step_x
    }

    /// Return the step in the Y color component.
    #[must_use]
    pub const fn step_y(&self) -> i16 {
        self.step_y
    }

    /// Return the transition time.
    #[must_use]
    pub fn transition_time(&self) -> Duration {
        Duration::from_deci_seconds(self.transition_time)
    }

    /// Return the options for this command.
    #[must_use]
    pub const fn options(&self) -> Options {
        self.options
    }
}

impl Cluster for StepColor {
    const ID: u16 = CLUSTER_ID;
}

impl Command for StepColor {
    const ID: u8 = 0x09;
    const DIRECTION: Direction = Direction::ClientToServer;
}
