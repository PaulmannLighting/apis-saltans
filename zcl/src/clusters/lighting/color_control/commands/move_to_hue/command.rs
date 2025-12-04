use core::time::Duration;

use zigbee::constants::DECI_SECONDS_PER_MILLISECOND;

use crate::clusters::lighting::color_control::CLUSTER_ID;
use crate::clusters::lighting::color_control::move_to_hue::Direction;
use crate::{Cluster, Command};

/// Command to move a light to a specific hue.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MoveToHue {
    hue: u8,
    direction: Direction,
    transition_time: u16,
}

impl MoveToHue {
    /// Create a new `MoveToHue` command.
    #[must_use]
    pub const fn new(hue: u8, direction: Direction, transition_time: u16) -> Self {
        Self {
            hue,
            direction,
            transition_time,
        }
    }

    /// Return the hue value.
    #[must_use]
    pub const fn hue(self) -> u8 {
        self.hue
    }

    /// Return the direction of the hue move.
    #[must_use]
    pub const fn direction(self) -> Direction {
        self.direction
    }

    /// Return the transition time in deci-seconds.
    #[must_use]
    pub fn transition_time(self) -> Duration {
        Duration::from_millis(u64::from(self.transition_time) * DECI_SECONDS_PER_MILLISECOND)
    }
}

impl Cluster for MoveToHue {
    const ID: u16 = CLUSTER_ID;
}

impl Command for MoveToHue {
    const ID: u8 = 0x00;
    const DIRECTION: zigbee::Direction = zigbee::Direction::ClientToServer;
}
