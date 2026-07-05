use apis_saltans_core::types::Uint16;
use apis_saltans_core::{ClusterId, Direction};

use crate::Options;
use crate::macros::zcl_command;

zcl_command! {
    /// Command to move a light to a specific hue and saturation with enhanced precision.
    EnhancedMoveToHueAndSaturation {
        { ClusterId::ColorControl } => ColorControl;
        command_id: 0x43;
        direction: Direction::ClientToServer;
        fields {
            enhanced_hue: u16,
            saturation: u8,
            transition_time: Uint16,
            options: Options,
        }

        getters {
            /// Return the enhanced hue value.
            #[must_use]
            pub const fn enhanced_hue(&self) -> u16 {
                self.enhanced_hue
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
