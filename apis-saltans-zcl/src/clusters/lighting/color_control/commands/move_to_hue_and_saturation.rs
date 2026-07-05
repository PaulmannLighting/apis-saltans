use apis_saltans_core::types::Uint16;
use apis_saltans_core::{ClusterId, Direction};

use crate::Options;
use crate::macros::zcl_command;

zcl_command! {
    /// Command to move a light to a specific hue and saturation.
    MoveToHueAndSaturation {
        { ClusterId::ColorControl } => ColorControl;
        command_id: 0x06;
        direction: Direction::ClientToServer;
        fields {
            hue: u8,
            saturation: u8,
            transition_time: Uint16,
            options: Options,
        }

        getters {
            /// Return the hue value.
            #[must_use]
            pub const fn hue(&self) -> u8 {
                self.hue
            }

            /// Return the saturation value.
            #[must_use]
            pub const fn saturation(&self) -> u8 {
                self.saturation
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
