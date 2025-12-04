use core::time::Duration;

use zigbee::constants::DECI_SECONDS_PER_MILLISECOND;

use crate::clusters::lighting::color_control::CLUSTER_ID;
use crate::clusters::lighting::color_control::move_to_hue::Direction;
use crate::{Cluster, Command};

/// Command to move a light to a specific extended hue with a direction and transition time.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct EnhancedMoveToHue {
    enhanced_hue: u16,
    direction: Direction,
    transition_time: u16,
}

impl EnhancedMoveToHue {
    /// Create a new `EnhancedMoveToHue` command.
    #[must_use]
    pub const fn new(enhanced_hue: u16, direction: Direction, transition_time: u16) -> Self {
        Self {
            enhanced_hue,
            direction,
            transition_time,
        }
    }

    /// Return the enhanced hue value.
    #[must_use]
    pub const fn enhanced_hue(self) -> u16 {
        self.enhanced_hue
    }

    /// Return the direction of the hue change.
    #[must_use]
    pub const fn direction(self) -> Direction {
        self.direction
    }

    /// Return the transition time.
    #[must_use]
    pub fn transition_time(self) -> Duration {
        Duration::from_millis(u64::from(self.transition_time) * DECI_SECONDS_PER_MILLISECOND)
    }
}

impl Cluster for EnhancedMoveToHue {
    const ID: u16 = CLUSTER_ID;
}

impl Command for EnhancedMoveToHue {
    const ID: u8 = 0x40;
    const DIRECTION: zigbee::Direction = zigbee::Direction::ClientToServer;
}
