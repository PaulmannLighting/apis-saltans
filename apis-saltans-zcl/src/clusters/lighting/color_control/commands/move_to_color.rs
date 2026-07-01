use le_stream::{FromLeStream, ToLeStream};
use apis_saltans_core::types::Uint16;
use apis_saltans_core::units::Deciseconds;
use apis_saltans_core::{ClusterId, Cluster, Direction};

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
        transition_time: Deciseconds,
        options: Options,
    ) -> Self {
        Self {
            color_x,
            color_y,
            transition_time: transition_time.into_inner(),
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
    pub fn transition_time(&self) -> Option<Deciseconds> {
        self.transition_time.try_into().ok()
    }

    /// Return the options.
    #[must_use]
    pub const fn options(&self) -> Options {
        self.options
    }
}

impl Cluster<ClusterId> for MoveToColor {
    const ID: ClusterId = ClusterId::ColorControl;
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
