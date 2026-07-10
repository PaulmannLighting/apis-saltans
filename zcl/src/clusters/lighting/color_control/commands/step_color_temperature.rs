use num_traits::FromPrimitive;
use zb_core::types::Uint16;
use zb_core::{Cluster, Direction};

use crate::Options;
use crate::clusters::lighting::color_control::step_hue::Mode;
use crate::macros::zcl_command;

zcl_command! {
    /// Command to step a light's color temperature in a specified range.
    StepColorTemperature {
        { Cluster::ColorControl } => ColorControl;
        command_id: 0x4c;
        direction: Direction::ClientToServer;
        fields {
            mode: u8,
            size: u16,
            transition_time: Uint16,
            color_temp_min_mireds: u16,
            color_temp_max_mireds: u16,
            options: Options,
        }

        constructor {
            /// Create a new `StepColorTemperature` command.
            #[must_use]
            pub const fn new(
                mode: Mode,
                size: u16,
                transition_time: Uint16,
                color_temp_min_mireds: u16,
                color_temp_max_mireds: u16,
                options: Options,
            ) -> Self {
                Self {
                    mode: mode as u8,
                    size,
                    transition_time,
                    color_temp_min_mireds,
                    color_temp_max_mireds,
                    options,
                }
            }
        }

        getters {
            /// Return the mode of color temperature step.
            ///
            /// # Errors
            ///
            /// Returns the raw mode value if it cannot be converted into a `Mode` enum.
            pub fn mode(&self) -> Result<Mode, u8> {
                Mode::from_u8(self.mode).ok_or(self.mode)
            }

            /// Return the size of color temperature step.
            #[must_use]
            pub const fn size(&self) -> u16 {
                self.size
            }

            /// Return the transition time, if any, in deciseconds.
            #[must_use]
            pub fn transition_time(&self) -> Option<u16> {
                self.transition_time.into()
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
