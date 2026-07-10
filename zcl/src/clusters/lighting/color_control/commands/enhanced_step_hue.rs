use num_traits::FromPrimitive;
use zb_core::types::Uint16;
use zb_core::{Cluster, Direction};

use crate::Options;
use crate::clusters::lighting::color_control::step_hue::Mode;
use crate::macros::zcl_command;

zcl_command! {
    /// Command to step a light's hue in an enhanced way, allowing for more control over the size.
    EnhancedStepHue {
        { Cluster::ColorControl } => ColorControl;
        command_id: 0x42;
        direction: Direction::ClientToServer;
        fields {
            mode: u8,
            size: u16,
            transition_time: Uint16,
            options: Options,
        }

        constructor {
            /// Create a new `EnhancedStepHue` command.
            #[must_use]
            pub const fn new(mode: Mode, size: u16, transition_time: Uint16, options: Options) -> Self {
                Self {
                    mode: mode as u8,
                    size,
                    transition_time,
                    options,
                }
            }
        }

        getters {
            /// Return the mode of hue step.
            ///
            /// # Errors
            ///
            /// Returns the raw mode value if it cannot be converted into a `Mode` enum.
            pub fn mode(&self) -> Result<Mode, u8> {
                Mode::from_u8(self.mode).ok_or(self.mode)
            }

            /// Return the size of hue step.
            #[must_use]
            pub const fn size(&self) -> u16 {
                self.size
            }

            /// Return the transition time in deci-seconds.
            #[must_use]
            pub fn transition_time(&self) -> Option<u16> {
                self.transition_time.into()
            }

            /// Return the options for this command.
            #[must_use]
            pub const fn options(&self) -> Options {
                self.options
            }
        }
    }
}
