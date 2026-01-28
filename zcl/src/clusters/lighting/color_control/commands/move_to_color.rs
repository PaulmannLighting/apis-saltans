use core::time::Duration;

use le_stream::ToLeStream;
use zigbee::{Cluster, Direction, FromDeciSeconds};

use crate::Command;
use crate::clusters::lighting::color_control::CLUSTER_ID;
use crate::options::Options;

/// Command to move a light to a specific color.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ToLeStream)]
pub struct MoveToColor {
    color_x: u16,
    color_y: u16,
    transition_time: u16,
    options: Options,
}

impl MoveToColor {
    /// Create a new `MoveToColor` command.
    #[must_use]
    pub const fn new(color_x: u16, color_y: u16, transition_time: u16, options: Options) -> Self {
        Self {
            color_x,
            color_y,
            transition_time,
            options,
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
        Duration::from_deci_seconds(self.transition_time)
    }

    /// Return the options.
    #[must_use]
    pub const fn options(self) -> Options {
        self.options
    }
}

impl Cluster for MoveToColor {
    const ID: u16 = CLUSTER_ID;
}

impl Command for MoveToColor {
    const ID: u8 = 0x07;
    const DIRECTION: Direction = Direction::ClientToServer;
}
