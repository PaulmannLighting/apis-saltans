use std::time::Duration;

use crate::zcl::{
    Cluster, Command, constants::DECI_SECONDS_PER_MILLISECOND, lighting::move_to_hue::Direction,
};

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
    const ID: u16 = 0x0300;
}

impl Command for EnhancedMoveToHue {
    const ID: u8 = 0x40;
}
