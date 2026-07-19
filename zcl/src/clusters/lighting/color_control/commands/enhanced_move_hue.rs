use zb_core::{Cluster, Direction};

use crate::Options;
use crate::clusters::lighting::color_control::move_hue::Mode;
use crate::macros::zcl_command;

zcl_command! {
    /// Command to move a light's hue in an enhanced way, allowing for more control over the rate.
    EnhancedMoveHue {
        { Cluster::ColorControl } => ColorControl;
        command_id: 0x41;
        direction: Direction::ClientToServer;
        fields {
            mode: u8,
            rate: u16,
            options: Options,
        }

        constructor {
            /// Create a new `EnhancedMoveHue` command.
            #[must_use]
            pub const fn new(mode: Mode, rate: u16, options: Options) -> Self {
                Self {
                    mode: mode as u8,
                    rate,
                    options,
                }
            }
        }

        getters {
            /// Return the mode of hue movement.
            ///
            /// # Errors
            ///
            /// Returns the raw mode value if it does not correspond to a valid `Mode` variant.
            pub fn mode(&self) -> Result<Mode, u8> {
                Mode::try_from(self.mode).map_err(|_| self.mode)
            }

            /// Return the rate of hue change in steps per second.
            #[must_use]
            pub const fn rate(&self) -> u16 {
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
