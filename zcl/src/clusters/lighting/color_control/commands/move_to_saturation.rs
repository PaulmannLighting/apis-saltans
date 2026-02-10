use core::num::TryFromIntError;
use core::time::Duration;

use zigbee::{Cluster, Direction, FromDeciSeconds, IntoDeciSeconds};

use crate::clusters::lighting::color_control::CLUSTER_ID;
use crate::{Command, Options};

/// Command to move a light to a specific saturation.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MoveToSaturation {
    saturation: u8,
    transition_time: u16,
    options: Options,
}

impl MoveToSaturation {
    /// Create a new `MoveToSaturation` command.
    #[must_use]
    pub const fn new(saturation: u8, transition_time: u16, options: Options) -> Self {
        Self {
            saturation,
            transition_time,
            options,
        }
    }

    /// Try to create a new `MoveToSaturation` command.
    ///
    /// # Errors
    ///
    /// Returns an [`TryFromIntError`] if the resulting deci-seconds value cannot fit in a `u16`.
    fn try_new(
        saturation: u8,
        transition_time: Duration,
        options: Options,
    ) -> Result<Self, TryFromIntError> {
        transition_time
            .into_deci_seconds()
            .try_into()
            .map(|transition_time| Self::new(saturation, transition_time, options))
    }

    /// Return the saturation value.
    #[must_use]
    pub const fn saturation(&self) -> u8 {
        self.saturation
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

impl Cluster for MoveToSaturation {
    const ID: u16 = CLUSTER_ID;
}

impl Command for MoveToSaturation {
    const ID: u8 = 0x03;
    const DIRECTION: Direction = Direction::ClientToServer;
}
