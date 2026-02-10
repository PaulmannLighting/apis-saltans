//! Data structures for the `Step Hue` command in the `Lighting` cluster.

use core::num::TryFromIntError;
use core::time::Duration;

use zigbee::{Cluster, Direction, FromDeciSeconds, IntoDeciSeconds};

pub use self::mode::Mode;
use crate::lighting::color_control::CLUSTER_ID;
use crate::{Command, Options};

mod mode;

/// Command to step a light's hue.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct StepHue {
    mode: Mode,
    size: u8,
    transition_time: u8,
    options: Options,
}

impl StepHue {
    /// Create a new `StepHue` command.
    #[must_use]
    pub const fn new(mode: Mode, size: u8, transition_time: u8, options: Options) -> Self {
        Self {
            mode,
            size,
            transition_time,
            options,
        }
    }

    /// Try to crate a new `StepHue` command.
    ///
    /// # Errors
    ///
    /// Returns an [`TryFromIntError`] if the resulting deci-seconds value cannot fit in a `u8`.
    pub fn try_new(
        mode: Mode,
        size: u8,
        transition_time: Duration,
        options: Options,
    ) -> Result<Self, TryFromIntError> {
        transition_time
            .into_deci_seconds()
            .try_into()
            .map(|transition_time| Self::new(mode, size, transition_time, options))
    }

    /// Return the misc of hue step.
    #[must_use]
    pub const fn mode(&self) -> Mode {
        self.mode
    }

    /// Return the size of hue step.
    #[must_use]
    pub const fn size(&self) -> u8 {
        self.size
    }

    /// Return the transition time in deci-seconds.
    #[must_use]
    pub fn transition_time(&self) -> Duration {
        Duration::from_deci_seconds(self.transition_time.into())
    }

    /// Return the options for the command.
    #[must_use]
    pub const fn options(&self) -> Options {
        self.options
    }
}

impl Cluster for StepHue {
    const ID: u16 = CLUSTER_ID;
}

impl Command for StepHue {
    const ID: u8 = 0x02;
    const DIRECTION: Direction = Direction::ClientToServer;
}
