use apis_saltans_core::types::Uint16;
use apis_saltans_core::units::Deciseconds;
use apis_saltans_core::{ClusterId, Direction};

use crate::macros::zcl_command;
use crate::options::Options;

zcl_command! {
    /// Move to level command.
    MoveToLevel {
        { ClusterId::Level } => Level;
        command_id: 0x00;
        direction: Direction::ClientToServer;
        derive(Copy);
        fields {
            level: u8,
            transition_time: Uint16,
            options: Options,
        }

        constructor {
            /// Creates a new `MoveToLevel` command.
            #[must_use]
            pub const fn new(level: u8, transition_time: Deciseconds, options: Options) -> Self {
                Self {
                    level,
                    transition_time: transition_time.into_inner(),
                    options,
                }
            }
        }

        getters {
            /// Get the level.
            #[must_use]
            pub const fn level(self) -> u8 {
                self.level
            }

            /// Return the transition time, if any, in deciseconds.
            #[must_use]
            pub fn transition_time(self) -> Option<Deciseconds> {
                self.transition_time.try_into().ok()
            }

            /// Get the options.
            #[must_use]
            pub const fn options(self) -> Options {
                self.options
            }
        }
    }
}
