use le_stream::{FromLeStream, ToLeStream};
use zigbee::{Cluster, Direction};

use crate::{ClusterId, Command, Native, Options};

/// Command to move a light's color.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct MoveColor {
    rate_x: i16,
    rate_y: i16,
    options: Options,
}

impl MoveColor {
    /// Create a new `MoveColor` command.
    #[must_use]
    pub const fn new(rate_x: i16, rate_y: i16, options: Options) -> Self {
        Self {
            rate_x,
            rate_y,
            options,
        }
    }

    /// Return the rate of change in the X color component.
    #[must_use]
    pub const fn rate_x(&self) -> i16 {
        self.rate_x
    }

    /// Return the rate of change in the Y color component.
    #[must_use]
    pub const fn rate_y(&self) -> i16 {
        self.rate_y
    }

    /// Return the options for the command.
    #[must_use]
    pub const fn options(&self) -> Options {
        self.options
    }
}

impl Cluster for MoveColor {
    const ID: u16 = ClusterId::ColorControl.as_u16();
}

impl Command for MoveColor {
    const ID: u8 = 0x08;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl Native for MoveColor {}

impl From<MoveColor> for crate::Cluster {
    fn from(command: MoveColor) -> Self {
        Self::ColorControl(command.into())
    }
}
