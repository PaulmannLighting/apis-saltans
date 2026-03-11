//! Data structures for the `Color Loop Set` command in the `Lighting` cluster.

use core::time::Duration;

use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use zigbee::Cluster;

pub use self::action::{Action, Source};
pub use self::direction::Direction;
pub use self::update::Update;
use crate::lighting::color_control::CLUSTER_ID;
use crate::{Command, Options};

mod action;
mod direction;
mod update;

/// Activate a light's color loop.
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct ColorLoopSet {
    update: Update,
    action: u8,
    direction: u8,
    time: u16,
    start_hue: u16,
    options: Options,
}

impl ColorLoopSet {
    /// Create a new `ColorLoopSet` command.
    #[must_use]
    pub const fn new(
        update: Update,
        action: Action,
        direction: Direction,
        time: u16,
        start_hue: u16,
        options: Options,
    ) -> Self {
        Self {
            update,
            action: action.as_u8(),
            direction: direction as u8,
            time,
            start_hue,
            options,
        }
    }

    /// Return the update mode.
    #[must_use]
    pub const fn update(self) -> Update {
        self.update
    }

    /// Return the action to perform.
    ///
    /// # Errors
    ///
    /// Returns the raw `u8` value if it does not correspond to a valid `Action`.
    pub fn action(self) -> Result<Action, u8> {
        Action::from_u8(self.action).ok_or(self.action)
    }

    /// Return the direction of the color loop.
    ///
    /// # Errors
    ///
    /// Returns the raw `u8` value if the direction is invalid.
    pub fn direction(self) -> Result<Direction, u8> {
        Direction::from_u8(self.direction).ok_or(self.direction)
    }

    /// Return the time.
    #[must_use]
    pub fn time(self) -> Duration {
        Duration::from_secs(u64::from(self.time))
    }

    /// Return the starting hue value.
    #[must_use]
    pub const fn start_hue(self) -> u16 {
        self.start_hue
    }

    /// Return the options for this command.
    #[must_use]
    pub const fn options(self) -> Options {
        self.options
    }
}

impl Cluster for ColorLoopSet {
    const ID: u16 = CLUSTER_ID;
}

impl Command for ColorLoopSet {
    const ID: u8 = 0x44;
    const DIRECTION: zigbee::Direction = zigbee::Direction::ClientToServer;
}
