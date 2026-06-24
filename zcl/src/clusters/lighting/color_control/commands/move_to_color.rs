use le_stream::{FromLeStream, ToLeStream};
use zigbee::types::Uint16;
use zigbee::{ClusterId, ClusterSpecific, Direction};

use crate::Command;
use crate::options::Options;

/// Command to move a light to a specific color.
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct MoveToColor {
    color_x: u16,
    color_y: u16,
    transition_time: Uint16,
    options: Options,
}

impl MoveToColor {
    /// Create a new `MoveToColor` command.
    #[must_use]
    pub const fn new(
        color_x: u16,
        color_y: u16,
        transition_time: Uint16,
        options: Options,
    ) -> Self {
        Self {
            color_x,
            color_y,
            transition_time,
            options,
        }
    }

    /// Return the color X value.
    #[must_use]
    pub const fn color_x(&self) -> u16 {
        self.color_x
    }

    /// Return the color Y value.
    #[must_use]
    pub const fn color_y(&self) -> u16 {
        self.color_y
    }

    /// Return the transition time, if any, in deciseconds.
    #[must_use]
    pub fn transition_time(&self) -> Option<u16> {
        self.transition_time.into()
    }

    /// Return the options.
    #[must_use]
    pub const fn options(&self) -> Options {
        self.options
    }
}

impl ClusterSpecific for MoveToColor {
    const CLUSTER: ClusterId = ClusterId::ColorControl;
}

impl Command for MoveToColor {
    const ID: u8 = 0x07;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl From<MoveToColor> for crate::Cluster {
    fn from(command: MoveToColor) -> Self {
        Self::ColorControl(command.into())
    }
}
