use std::time::Duration;

use num_derive::FromPrimitive;

use crate::zcl::{Cluster, Command, constants::DECI_SECONDS_PER_MILLISECOND};

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
    const ID: u16 = 0x0300;
}

impl Command for MoveToHue {
    const ID: u8 = 0x00;
}

/// Direction of hue flow.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Direction {
    /// Take the shortest distance.
    ShortestDistance = 0x00,
    /// Take the longest distance.
    LongestDistance = 0x01,
    /// Move up.
    Up = 0x02,
    /// Move down.
    Down = 0x03,
}
