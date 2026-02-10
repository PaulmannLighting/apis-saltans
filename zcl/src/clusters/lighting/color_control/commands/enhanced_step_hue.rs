use core::num::TryFromIntError;
use core::time::Duration;

use zigbee::{Cluster, Direction, FromDeciSeconds, IntoDeciSeconds};

use crate::clusters::lighting::color_control::CLUSTER_ID;
use crate::clusters::lighting::color_control::step_hue::Mode;
use crate::{Command, Options};

/// Command to step a light's hue in an enhanced way, allowing for more control over the size.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EnhancedStepHue {
    mode: Mode,
    size: u16,
    transition_time: u16,
    options: Options,
}

impl EnhancedStepHue {
    /// Create a new `EnhancedStepHue` command.
    #[must_use]
    pub const fn new(mode: Mode, size: u16, transition_time: u16, options: Options) -> Self {
        Self {
            mode,
            size,
            transition_time,
            options,
        }
    }

    /// Try to create a new `EnhancedStepHue` command.
    ///
    /// # Errors
    ///
    /// Returns an [`TryFromIntError`] if the resulting deci-seconds value cannot fit in a `u16`.
    pub fn try_new(
        mode: Mode,
        size: u16,
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
    pub const fn size(&self) -> u16 {
        self.size
    }

    /// Return the transition time in deci-seconds.
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

impl Cluster for EnhancedStepHue {
    const ID: u16 = CLUSTER_ID;
}

impl Command for EnhancedStepHue {
    const ID: u8 = 0x42;
    const DIRECTION: Direction = Direction::ClientToServer;
}
