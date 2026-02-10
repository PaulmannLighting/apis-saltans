//! Data structures for the `Move Hue` command in the `Lighting` cluster.

use zigbee::{Cluster, Direction};

pub use self::mode::Mode;
use crate::lighting::color_control::CLUSTER_ID;
use crate::{Command, Options};

mod mode;

/// Command to move a light's hue.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MoveHue {
    mode: Mode,
    rate: u8,
    options: Options,
}

impl MoveHue {
    /// Create a new `MoveHue` command.
    #[must_use]
    pub const fn new(mode: Mode, rate: u8, options: Options) -> Self {
        Self {
            mode,
            rate,
            options,
        }
    }

    /// Return the misc.
    #[must_use]
    pub const fn mode(&self) -> Mode {
        self.mode
    }

    /// Return the rate of hue change in steps per second.
    #[must_use]
    pub const fn rate(&self) -> u8 {
        self.rate
    }

    /// Return the options for the command.
    #[must_use]
    pub const fn options(&self) -> Options {
        self.options
    }
}

impl Cluster for MoveHue {
    const ID: u16 = CLUSTER_ID;
}

impl Command for MoveHue {
    const ID: u8 = 0x01;
    const DIRECTION: Direction = Direction::ClientToServer;
}
