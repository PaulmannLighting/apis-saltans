use le_stream::{FromLeStream, ToLeStream};
use apis_saltans_core::types::Uint16;
use apis_saltans_core::{ClusterId, ClusterSpecific, Direction};

use crate::{Command, Options};

/// Command to step a light's color.
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct StepColor {
    step_x: i16,
    step_y: i16,
    transition_time: Uint16,
    options: Options,
}

impl StepColor {
    /// Create a new `StepColor` command.
    #[must_use]
    pub const fn new(step_x: i16, step_y: i16, transition_time: Uint16, options: Options) -> Self {
        Self {
            step_x,
            step_y,
            transition_time,
            options,
        }
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

    /// Return the transition time, if any, in deciseconds.
    #[must_use]
    pub fn transition_time(&self) -> Option<u16> {
        self.transition_time.into()
    }

    /// Return the options for this command.
    #[must_use]
    pub const fn options(&self) -> Options {
        self.options
    }
}

impl ClusterSpecific for StepColor {
    const CLUSTER: ClusterId = ClusterId::ColorControl;
}

impl Command for StepColor {
    const ID: u8 = 0x09;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl From<StepColor> for crate::Cluster {
    fn from(command: StepColor) -> Self {
        Self::ColorControl(command.into())
    }
}
