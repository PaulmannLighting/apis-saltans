use zb_core::types::Uint16;
use zb_core::units::{Deciseconds, Mireds};
use zb_core::{Cluster, Direction};

use crate::Options;
use crate::macros::zcl_command;

zcl_command! {
    /// Command to move a light's color temperature to a specific value in mireds.
    MoveToColorTemperature {
        { Cluster::ColorControl } => ColorControl;
        command_id: 0x0a;
        direction: Direction::ClientToServer;
        fields {
            mireds: u16,
            transition_time: Uint16,
            options: Options,
        }

        constructor {
            /// Create a new `MoveToColorTemperature` command.
            #[must_use]
            pub fn new(
                mireds: Mireds,
                transition_time: Deciseconds,
                options: Options,
            ) -> Self {
                Self {
                    mireds: mireds.into(),
                    transition_time: transition_time.into(),
                    options,
                }
            }
        }

        getters {
            /// Return the color temperature in mireds.
            ///
            /// # Errors
            ///
            /// Returns the raw value if it is not a valid [`Mireds`] value.
            pub fn mireds(&self) -> Result<Mireds, u16> {
                self.mireds.try_into()
            }

            /// Return the transition time, if any, in deciseconds.
            #[must_use]
            pub fn transition_time(&self) -> Option<Deciseconds> {
                Deciseconds::new(self.transition_time)
            }

            /// Return the options for this command.
            #[must_use]
            pub const fn options(&self) -> Options {
                self.options
            }
        }
    }
}
