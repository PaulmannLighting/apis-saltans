//! Data structures for the `Move To Hue` command in the `Lighting` cluster.

use core::num::TryFromIntError;
use core::time::Duration;

use zigbee::{Cluster, FromDeciSeconds, IntoDeciSeconds};

pub use self::direction::Direction;
use crate::lighting::color_control::CLUSTER_ID;
use crate::{Command, Options};

mod direction;

/// Command to move a light to a specific hue.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MoveToHue {
    hue: u8,
    direction: Direction,
    transition_time: u16,
    options: Options,
}

impl MoveToHue {
    /// Create a new `MoveToHue` command.
    #[must_use]
    pub const fn new(
        hue: u8,
        direction: Direction,
        transition_time: u16,
        options: Options,
    ) -> Self {
        Self {
            hue,
            direction,
            transition_time,
            options,
        }
    }

    /// Try to create a new `MoveToHue` command.
    ///
    /// # Errors
    ///
    /// Returns an [`TryFromIntError`] if the resulting deci-seconds value cannot fit in a `u16`.
    pub fn try_new(
        hue: u8,
        direction: Direction,
        transition_time: Duration,
        options: Options,
    ) -> Result<Self, TryFromIntError> {
        transition_time
            .into_deci_seconds()
            .try_into()
            .map(|transition_time| Self::new(hue, direction, transition_time, options))
    }

    /// Return the hue value.
    #[must_use]
    pub const fn hue(&self) -> u8 {
        self.hue
    }

    /// Return the direction of the hue move.
    #[must_use]
    pub const fn direction(&self) -> Direction {
        self.direction
    }

    /// Return the transition time in deci-seconds.
    #[must_use]
    pub fn transition_time(&self) -> Duration {
        Duration::from_deci_seconds(self.transition_time)
    }

    /// Return the options for the command.
    #[must_use]
    pub const fn options(&self) -> Options {
        self.options
    }
}

impl Cluster for MoveToHue {
    const ID: u16 = CLUSTER_ID;
}

impl Command for MoveToHue {
    const ID: u8 = 0x00;
    const DIRECTION: zigbee::Direction = zigbee::Direction::ClientToServer;
}
