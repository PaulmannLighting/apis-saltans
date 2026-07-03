use apis_saltans_core::types::Uint16;
use apis_saltans_core::{ClusterId, Direction};

use crate::Options;
use crate::macros::zcl_command;

zcl_command! {
    /// Command to move a light's color temperature to a specific value in mireds.
    MoveToColorTemperature {
        { ClusterId::ColorControl } => ColorControl;
        command_id: 0x0a;
        direction: Direction::ClientToServer;
        => super::MoveToColorTemperature;
        fields {
            mireds: u16,
            transition_time: Uint16,
            options: Options,
        }

        getters {
            /// Return the color temperature in mireds.
            #[must_use]
            pub const fn mireds(&self) -> u16 {
                self.mireds
            }

            /// Return the transition time, if any, in deciseconds.
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
