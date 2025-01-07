use num_derive::FromPrimitive;

use crate::zcl::cluster::Cluster;
use crate::zcl::command::Command;

/// Command to move a light to a specific huw.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MoveToHue {
    hue: u8,
    direction: Direction,
    transition_time: u16,
}

impl MoveToHue {
    #[must_use]
    pub const fn new(hue: u8, direction: Direction, transition_time: u16) -> Self {
        Self {
            hue,
            direction,
            transition_time,
        }
    }

    #[must_use]
    pub const fn hue(&self) -> u8 {
        self.hue
    }

    #[must_use]
    pub const fn direction(&self) -> Direction {
        self.direction
    }

    #[must_use]
    pub const fn transition_time(&self) -> u16 {
        self.transition_time
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
