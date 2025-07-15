use std::time::Duration;

use crate::zcl::Command;
use crate::zcl::constants::DECI_SECONDS_PER_MILLISECOND;
use crate::zcl::lighting::Lighting;

/// Command to move a light to a specific color.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MoveToColor {
    color_x: u16,
    color_y: u16,
    transition_time: u16,
}

impl MoveToColor {
    /// Create a new `MoveToColor` command.
    #[must_use]
    pub const fn new(color_x: u16, color_y: u16, transition_time: u16) -> Self {
        Self {
            color_x,
            color_y,
            transition_time,
        }
    }

    /// Return the color X value.
    #[must_use]
    pub const fn color_x(self) -> u16 {
        self.color_x
    }

    /// Return the color Y value.
    #[must_use]
    pub const fn color_y(self) -> u16 {
        self.color_y
    }

    /// Return the transition time.
    #[must_use]
    pub fn transition_time(self) -> Duration {
        Duration::from_millis(u64::from(self.transition_time) * DECI_SECONDS_PER_MILLISECOND)
    }
}

impl Lighting for MoveToColor {}

impl Command for MoveToColor {
    const ID: u8 = 0x07;
}
