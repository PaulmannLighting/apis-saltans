use core::num::TryFromIntError;
use core::time::Duration;

use zigbee::{Cluster, Direction, FromDeciSeconds, IntoDeciSeconds};

use crate::clusters::lighting::color_control::CLUSTER_ID;
use crate::{Command, Options};

/// Command to move a light to a specific hue and saturation.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MoveToHueAndSaturation {
    hue: u8,
    saturation: u8,
    transition_time: u16,
    options: Options,
}

impl MoveToHueAndSaturation {
    /// Create a new `MoveToHueAndSaturation` command.
    #[must_use]
    pub const fn new(hue: u8, saturation: u8, transition_time: u16, options: Options) -> Self {
        Self {
            hue,
            saturation,
            transition_time,
            options,
        }
    }

    /// Try to create a new `MoveToHueAndSaturation` command.
    ///
    /// # Errors
    ///
    /// Returns an [`TryFromIntError`] if the resulting deci-seconds value cannot fit in a `u16`.
    pub fn try_new(
        hue: u8,
        saturation: u8,
        transition_time: Duration,
        options: Options,
    ) -> Result<Self, TryFromIntError> {
        transition_time
            .into_deci_seconds()
            .try_into()
            .map(|transition_time| Self::new(hue, saturation, transition_time, options))
    }

    /// Return the hue value.
    #[must_use]
    pub const fn hue(&self) -> u8 {
        self.hue
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

impl Cluster for MoveToHueAndSaturation {
    const ID: u16 = CLUSTER_ID;
}

impl Command for MoveToHueAndSaturation {
    const ID: u8 = 0x06;
    const DIRECTION: Direction = Direction::ClientToServer;
}
