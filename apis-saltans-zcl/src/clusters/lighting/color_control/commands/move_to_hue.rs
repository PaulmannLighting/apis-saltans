//! Data structures for the `Move To Hue` command in the `Lighting` cluster.

use apis_saltans_core::ClusterId;
use apis_saltans_core::types::Uint16;
use num_traits::FromPrimitive;

pub use self::direction::Direction;
use crate::Options;
use crate::macros::zcl_command;

mod direction;

zcl_command! {
    /// Command to move a light to a specific hue.
    MoveToHue {
        { ClusterId::ColorControl } => ColorControl;
        command_id: 0x00;
        direction: apis_saltans_core::Direction::ClientToServer;
        => super::MoveToHue;
        fields {
            hue: u8,
            direction: u8,
            transition_time: Uint16,
            options: Options,
        }

        constructor {
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
        }

        getters {
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
    }
}
