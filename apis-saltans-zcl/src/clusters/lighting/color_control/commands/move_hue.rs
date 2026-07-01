//! Data structures for the `Move Hue` command in the `Lighting` cluster.

use apis_saltans_core::{Cluster, ClusterId, Direction};
use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

pub use self::mode::Mode;
use crate::{Command, Options};

mod mode;

/// Command to move a light's hue.
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct MoveHue {
    mode: u8,
    rate: u8,
    options: Options,
}

impl MoveHue {
    /// Create a new `MoveHue` command.
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

impl Cluster<ClusterId> for MoveHue {
    const ID: ClusterId = ClusterId::ColorControl;
}

impl Command for MoveHue {
    const ID: u8 = 0x01;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl From<MoveHue> for crate::Cluster {
    fn from(command: MoveHue) -> Self {
        Self::ColorControl(command.into())
    }
}
