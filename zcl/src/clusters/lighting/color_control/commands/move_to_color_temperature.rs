use core::num::TryFromIntError;
use core::time::Duration;

use le_stream::{FromLeStream, ToLeStream};
use zigbee::{Cluster, Direction, FromDeciSeconds, IntoDeciSeconds};

use crate::clusters::lighting::color_control::CLUSTER_ID;
use crate::{Command, Options};

/// Command to move a light's color temperature to a specific value in mireds.
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct MoveToColorTemperature {
    mireds: u16,
    transition_time: u16,
    options: Options,
}

impl MoveToColorTemperature {
    /// Create a new `MoveToColorTemperature` command.
    #[must_use]
    pub const fn new(mireds: u16, transition_time: u16, options: Options) -> Self {
        Self {
            mireds,
            transition_time,
            options,
        }
    }

    /// Try to create a new `MoveToColorTemperature` command.
    ///
    /// # Errors
    ///
    /// Returns an [`TryFromIntError`] if the resulting deci-seconds value cannot fit in a `u16`.
    pub fn try_new(
        mireds: u16,
        transition_time: Duration,
        options: Options,
    ) -> Result<Self, TryFromIntError> {
        transition_time
            .into_deci_seconds()
            .try_into()
            .map(|transition_time| Self::new(mireds, transition_time, options))
    }

    /// Return the color temperature in mireds.
    #[must_use]
    pub const fn mireds(&self) -> u16 {
        self.mireds
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

impl Cluster for MoveToColorTemperature {
    const ID: u16 = CLUSTER_ID;
}

impl Command for MoveToColorTemperature {
    const ID: u8 = 0x0a;
    const DIRECTION: Direction = Direction::ClientToServer;
}
