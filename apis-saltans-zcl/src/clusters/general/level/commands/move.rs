use apis_saltans_core::types::Uint8;
use apis_saltans_core::units::UnitsPerSecond;
use apis_saltans_core::{ClusterId, Direction};

use crate::general::level::Mode;
use crate::macros::zcl_command;
use crate::options::Options;

zcl_command! {
    /// Move command.
    Move {
        { ClusterId::Level } => Level;
        command_id: 0x01;
        direction: Direction::ClientToServer;
        => super::Move;
        derive(Copy);
        fields {
            mode: u8,
            rate: Uint8,
            options: Options,
        }

        constructor {
            /// Crate a new `Move` command.
            #[must_use]
            pub fn new(mode: Mode<UnitsPerSecond>, options: Options) -> Self {
                Self {
                    mode: mode.discriminant(),
                    rate: mode.into_stride().into_inner(),
                    options,
                }
            }
        }

        getters {
            /// Get the mode.
            #[must_use]
            pub fn mode(self) -> Option<Mode<UnitsPerSecond>> {
                Mode::new(self.mode, self.rate()?).ok()
            }

            /// Get the rate.
            #[must_use]
            pub fn rate(self) -> Option<UnitsPerSecond> {
                self.rate.try_into().ok()
            }

            /// Get the options.
            #[must_use]
            pub const fn options(self) -> Options {
                self.options
            }
        }
    }
}
