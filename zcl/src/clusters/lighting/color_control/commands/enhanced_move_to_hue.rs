use core::num::TryFromIntError;
use core::time::Duration;

use zigbee::{Cluster, FromDeciSeconds, IntoDeciSeconds};

use crate::clusters::lighting::color_control::CLUSTER_ID;
use crate::clusters::lighting::color_control::move_to_hue::Direction;
use crate::{Command, Options};

/// Command to move a light to a specific extended hue with a direction and transition time.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EnhancedMoveToHue {
    enhanced_hue: u16,
    direction: Direction,
    transition_time: u16,
    options: Options,
}

impl EnhancedMoveToHue {
    /// Create a new `EnhancedMoveToHue` command.
    #[must_use]
    pub const fn new(
        enhanced_hue: u16,
        direction: Direction,
        transition_time: u16,
        options: Options,
    ) -> Self {
        Self {
            enhanced_hue,
            direction,
            transition_time,
            options,
        }
    }

    /// Try to create a new `EnhancedMoveToHue` command.
    ///
    /// # Errors
    ///
    /// Returns an [`TryFromIntError`] if the resulting deci-seconds value cannot fit in a `u16`.
    pub fn try_new(
        enhanced_hue: u16,
        direction: Direction,
        transition_time: Duration,
        options: Options,
    ) -> Result<Self, TryFromIntError> {
        transition_time
            .into_deci_seconds()
            .try_into()
            .map(|transition_time| Self::new(enhanced_hue, direction, transition_time, options))
    }

    /// Return the enhanced hue value.
    #[must_use]
    pub const fn enhanced_hue(&self) -> u16 {
        self.enhanced_hue
    }

    /// Return the direction of the hue change.
    #[must_use]
    pub const fn direction(&self) -> Direction {
        self.direction
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

impl Cluster for EnhancedMoveToHue {
    const ID: u16 = CLUSTER_ID;
}

impl Command for EnhancedMoveToHue {
    const ID: u8 = 0x40;
    const DIRECTION: zigbee::Direction = zigbee::Direction::ClientToServer;
}
