use apis_saltans_core::types::Uint16;
use apis_saltans_core::units::Deciseconds;
use apis_saltans_core::{ClusterId, Direction};

use crate::macros::zcl_command;
use crate::options::Options;

zcl_command! {
    /// Command to move a light to a specific color.
    MoveToColor {
        { ClusterId::ColorControl } => ColorControl;
        command_id: 0x07;
        direction: Direction::ClientToServer;
        fields {
            color_x: u16,
            color_y: u16,
            transition_time: Uint16,
            options: Options,
        }

        constructor {
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
        }

        getters {
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
    }
}
