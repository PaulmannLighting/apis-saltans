use core::time::Duration;

use le_stream::ToLeStream;
use zigbee::constants::DECI_SECONDS_PER_MILLISECOND;

use crate::clusters::lighting::color_control::CLUSTER_ID;
use crate::{Cluster, Command};

/// Command to move a light to a specific color.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ToLeStream)]
pub struct MoveToColor {
    color_x: u16,
    color_y: u16,
    transition_time: u16,
    options_mask: u8,
    options_override: u8,
}

impl MoveToColor {
    /// Create a new `MoveToColor` command.
    #[must_use]
    pub const fn new(
        color_x: u16,
        color_y: u16,
        transition_time: u16,
        options_mask: u8,
        options_override: u8,
    ) -> Self {
        Self {
            color_x,
            color_y,
            transition_time,
            options_mask,
            options_override,
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

    /// Return the options mask.
    #[must_use]
    pub const fn options_mask(self) -> u8 {
        self.options_mask
    }

    /// Return the options override.
    #[must_use]
    pub const fn options_override(self) -> u8 {
        self.options_override
    }
}

impl Cluster for MoveToColor {
    const ID: u16 = CLUSTER_ID;
}

impl Command for MoveToColor {
    const ID: u8 = 0x07;
}
