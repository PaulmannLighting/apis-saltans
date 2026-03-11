//! Data structures for the `Move Saturation` command in the `Lighting` cluster.

use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use zigbee::{Cluster, Direction};

pub use self::mode::Mode;
use crate::lighting::color_control::CLUSTER_ID;
use crate::{Command, Options};

mod mode;

/// Command to move a light's saturation.
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct MoveSaturation {
    mode: u8,
    rate: u8,
    options: Options,
}

impl MoveSaturation {
    /// Create a new `MoveSaturation` command.
    #[must_use]
    pub const fn new(mode: Mode, rate: u8, options: Options) -> Self {
        Self {
            mode: mode as u8,
            rate,
            options,
        }
    }

    /// Return the mode.
    ///
    /// # Errors
    ///
    /// Returns the raw mode value if it does not correspond to a valid `Mode` variant.
    pub fn mode(&self) -> Result<Mode, u8> {
        Mode::from_u8(self.mode).ok_or(self.mode)
    }

    /// Return the rate of saturation change in steps per second.
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

impl Cluster for MoveSaturation {
    const ID: u16 = CLUSTER_ID;
}

impl Command for MoveSaturation {
    const ID: u8 = 0x04;
    const DIRECTION: Direction = Direction::ClientToServer;
}
