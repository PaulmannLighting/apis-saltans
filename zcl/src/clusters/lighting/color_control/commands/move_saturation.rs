//! Data structures for the `Move Saturation` command in the `Lighting` cluster.

use zb_core::{Cluster, Direction};

pub use self::mode::Mode;
use crate::Options;
use crate::macros::zcl_command;

mod mode;

zcl_command! {
    /// Command to move a light's saturation.
    MoveSaturation {
        { Cluster::ColorControl } => ColorControl;
        command_id: 0x04;
        direction: Direction::ClientToServer;
        fields {
            mode: u8,
            rate: u8,
            options: Options,
        }

        constructor {
            /// Create a new `MoveSaturation` command.
            #[must_use]
            pub const fn new(mode: Mode, rate: u8, options: Options) -> Self {
                Self {
                    mode: mode as u8,
                    rate,
                    options,
                }
            }
        }

        getters {
            /// Return the mode.
            ///
            /// # Errors
            ///
            /// Returns the raw mode value if it does not correspond to a valid `Mode` variant.
            pub fn mode(&self) -> Result<Mode, u8> {
                Mode::try_from(self.mode).map_err(|_| self.mode)
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
    }
}
