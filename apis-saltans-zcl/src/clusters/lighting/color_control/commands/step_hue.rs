//! Data structures for the `Step Hue` command in the `Lighting` cluster.

use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use apis_saltans_core::{ClusterId, ClusterSpecific, Direction};

pub use self::mode::Mode;
use crate::{Command, Options};

mod mode;

/// Command to step a light's hue.
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct StepHue {
    mode: u8,
    size: u8,
    transition_time: u8,
    options: Options,
}

impl StepHue {
    /// Create a new `StepHue` command.
    #[must_use]
    pub const fn new(mode: Mode, size: u8, transition_time: u8, options: Options) -> Self {
        Self {
            mode: mode as u8,
            size,
            transition_time,
            options,
        }
    }

    /// Return the misc of hue step.
    ///
    /// # Errors
    ///
    /// Returns the raw mode value if it cannot be converted into a `Mode` enum.
    pub fn mode(&self) -> Result<Mode, u8> {
        Mode::from_u8(self.mode).ok_or(self.mode)
    }

    /// Return the size of hue step.
    #[must_use]
    pub const fn size(&self) -> u8 {
        self.size
    }

    /// Return the transition time in deci-seconds.
    #[must_use]
    pub const fn transition_time(&self) -> u8 {
        self.transition_time
    }

    /// Return the options for the command.
    #[must_use]
    pub const fn options(&self) -> Options {
        self.options
    }
}

impl ClusterSpecific for StepHue {
    const CLUSTER: ClusterId = ClusterId::ColorControl;
}

impl Command for StepHue {
    const ID: u8 = 0x02;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl From<StepHue> for crate::Cluster {
    fn from(command: StepHue) -> Self {
        Self::ColorControl(command.into())
    }
}
