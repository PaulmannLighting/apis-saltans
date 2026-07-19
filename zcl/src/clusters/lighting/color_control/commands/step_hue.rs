//! Data structures for the `Step Hue` command in the `Lighting` cluster.

use zb_core::{Cluster, Direction};

pub use self::mode::Mode;
use crate::Options;
use crate::macros::zcl_command;

mod mode;

zcl_command! {
    /// Command to step a light's hue.
    StepHue {
        { Cluster::ColorControl } => ColorControl;
        command_id: 0x02;
        direction: Direction::ClientToServer;
        fields {
            mode: u8,
            size: u8,
            transition_time: u8,
            options: Options,
        }

        constructor {
            /// Create a new `StepHue` command.
            #[must_use]
            pub const fn new(mode: Mode, size: u8, transition_time: u8, options: Options) -> Self {
                Self {
                    mode: mode as u8,
                    size,
                    transition_time,
                    options,
                }
            }
        }

        getters {
            /// Return the misc of hue step.
            ///
            /// # Errors
            ///
            /// Returns the raw mode value if it cannot be converted into a `Mode` enum.
            pub fn mode(&self) -> Result<Mode, u8> {
                Mode::try_from(self.mode).map_err(|_| self.mode)
            }

            /// Return the size of hue step.
            #[must_use]
            pub const fn size(&self) -> u8 {
                self.size
            }

            /// Return the transition time in deci-seconds.
            #[must_use]
            pub const fn transition_time(&self) -> u8 {
                self.transition_time
            }

            /// Return the options for the command.
            #[must_use]
            pub const fn options(&self) -> Options {
                self.options
            }
        }
    }
}
