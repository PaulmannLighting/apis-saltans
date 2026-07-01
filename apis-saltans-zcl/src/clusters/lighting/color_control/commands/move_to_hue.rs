//! Data structures for the `Move To Hue` command in the `Lighting` cluster.

use apis_saltans_core::types::Uint16;
use apis_saltans_core::{Cluster, ClusterId};
use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

pub use self::direction::Direction;
use crate::{Command, Options};

mod direction;

/// Command to move a light to a specific hue.
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct MoveToHue {
    hue: u8,
    direction: u8,
    transition_time: Uint16,
    options: Options,
}

impl MoveToHue {
    /// Create a new `MoveToHue` command.
    #[must_use]
    pub const fn new(
        hue: u8,
        direction: Direction,
        transition_time: Uint16,
        options: Options,
    ) -> Self {
        Self {
            hue,
            direction: direction as u8,
            transition_time,
            options,
        }
    }

    /// Return the hue value.
    #[must_use]
    pub const fn hue(&self) -> u8 {
        self.hue
    }

    /// Return the direction of the hue move.
    ///
    /// # Errors
    ///
    /// Returns an error if the direction value is not a valid `Direction`.
    pub fn direction(&self) -> Result<Direction, u8> {
        Direction::from_u8(self.direction).ok_or(self.direction)
    }

    /// Return the transition time, if any, in deciseconds.
    #[must_use]
    pub fn transition_time(&self) -> Option<u16> {
        self.transition_time.into()
    }

    /// Return the options for the command.
    #[must_use]
    pub const fn options(&self) -> Options {
        self.options
    }
}

impl Cluster<ClusterId> for MoveToHue {
    const ID: ClusterId = ClusterId::ColorControl;
}

impl Command for MoveToHue {
    const ID: u8 = 0x00;
    const DIRECTION: apis_saltans_core::Direction = apis_saltans_core::Direction::ClientToServer;
}

impl From<MoveToHue> for crate::Cluster {
    fn from(command: MoveToHue) -> Self {
        Self::ColorControl(command.into())
    }
}
