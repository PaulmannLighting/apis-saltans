use apis_saltans_core::{ClusterId, Direction};
use num_traits::FromPrimitive;

use crate::Options;
use crate::clusters::lighting::color_control::move_hue::Mode;
use crate::macros::zcl_command;

zcl_command! {
    /// Command to move a light's color temperature.
    MoveColorTemperature {
        { ClusterId::ColorControl } => ColorControl;
        command_id: 0x4b;
        direction: Direction::ClientToServer;
        => super::MoveColorTemperature;
        fields {
            mode: u8,
            rate: u16,
            color_temp_min_mireds: u16,
            color_temp_max_mireds: u16,
            options: Options,
        }

        constructor {
            /// Create a new `MoveColorTemperature` command.
            #[must_use]
            pub const fn new(
                mode: Mode,
                rate: u16,
                color_temp_min_mireds: u16,
                color_temp_max_mireds: u16,
                options: Options,
            ) -> Self {
                Self {
                    mode: mode as u8,
                    rate,
                    color_temp_min_mireds,
                    color_temp_max_mireds,
                    options,
                }
            }
        }

        getters {
            /// Return the mode of color temperature movement.
            ///
            /// # Errors
            ///
            /// Returns the raw mode value if it does not correspond to a valid `Mode` variant.
            pub fn mode(&self) -> Result<Mode, u8> {
                Mode::from_u8(self.mode).ok_or(self.mode)
            }

            /// Return the rate of color temperature change in mireds per second.
            #[must_use]
            pub const fn rate(&self) -> u16 {
                self.rate
            }

            /// Return the minimum color temperature in mireds.
            #[must_use]
            pub const fn color_temp_min_mireds(&self) -> u16 {
                self.color_temp_min_mireds
            }

            /// Return the maximum color temperature in mireds.
            #[must_use]
            pub const fn color_temp_max_mireds(&self) -> u16 {
                self.color_temp_max_mireds
            }

            /// Return the options for the command.
            #[must_use]
            pub const fn options(&self) -> Options {
                self.options
            }
        }
    }
}
