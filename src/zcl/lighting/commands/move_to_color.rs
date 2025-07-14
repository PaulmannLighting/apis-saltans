use std::time::Duration;

use crate::zcl::{Cluster, Command, constants::DECI_SECONDS_PER_MILLISECOND};

/// Command to move a light to a specific color.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MoveToColor {
    color_x: u16,
    color_y: u16,
    transition_time: u16,
}

impl MoveToColor {
    #[must_use]
    pub const fn new(color_x: u16, color_y: u16, transition_time: u16) -> Self {
        Self {
            color_x,
            color_y,
            transition_time,
        }
    }

    #[must_use]
    pub const fn color_x(self) -> u16 {
        self.color_x
    }

    #[must_use]
    pub const fn color_y(self) -> u16 {
        self.color_y
    }

    #[must_use]
    pub fn transition_time(self) -> Duration {
        Duration::from_millis(u64::from(self.transition_time) * DECI_SECONDS_PER_MILLISECOND)
    }
}

impl Cluster for MoveToColor {
    const ID: u16 = 0x0300;
}

impl Command for MoveToColor {
    const ID: u8 = 0x07;
}
