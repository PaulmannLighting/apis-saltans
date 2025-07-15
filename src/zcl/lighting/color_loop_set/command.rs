use std::time::Duration;

use crate::zcl::Command;
use crate::zcl::lighting::Lighting;
use crate::zcl::lighting::color_loop_set::{Action, Direction, Update};

/// Activate a light's color loop.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ColorLoopSet {
    update: Update,
    action: Action,
    direction: Direction,
    time: u16,
    start_hue: u16,
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
    ) -> Self {
        Self {
            update,
            action,
            direction,
            time,
            start_hue,
        }
    }

    /// Return the update mode.
    #[must_use]
    pub const fn update(self) -> Update {
        self.update
    }

    /// Return the action to perform.
    #[must_use]
    pub const fn action(self) -> Action {
        self.action
    }

    /// Return the direction of the color loop.
    #[must_use]
    pub const fn direction(self) -> Direction {
        self.direction
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
}

impl Lighting for ColorLoopSet {}

impl Command for ColorLoopSet {
    const ID: u8 = 0x44;
}
