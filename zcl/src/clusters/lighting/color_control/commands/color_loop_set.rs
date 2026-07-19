//! Data structures for the `Color Loop Set` command in the `Lighting` cluster.

use core::time::Duration;

use zb_core::Cluster;

pub use self::action::{Action, Source};
pub use self::direction::Direction;
pub use self::update::Update;
use crate::Options;
use crate::macros::zcl_command;

mod action;
mod direction;
mod update;

zcl_command! {
    /// Activate a light's color loop.
    ColorLoopSet {
        { Cluster::ColorControl } => ColorControl;
        command_id: 0x44;
        direction: zb_core::Direction::ClientToServer;
        fields {
            update: Update,
            action: u8,
            direction: u8,
            time: u16,
            start_hue: u16,
            options: Options,
        }

        constructor {
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
        }

        getters {
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
                Action::try_from(self.action)
            }

            /// Return the direction of the color loop.
            ///
            /// # Errors
            ///
            /// Returns the raw `u8` value if the direction is invalid.
            pub fn direction(self) -> Result<Direction, u8> {
                Direction::try_from(self.direction).map_err(|_| self.direction)
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
    }
}
