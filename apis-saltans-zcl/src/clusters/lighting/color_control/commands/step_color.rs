use apis_saltans_core::types::Uint16;
use apis_saltans_core::{ClusterId, Direction};

use crate::Options;
use crate::macros::zcl_command;

zcl_command! {
    /// Command to step a light's color.
    StepColor {
        { ClusterId::ColorControl } => ColorControl;
        command_id: 0x09;
        direction: Direction::ClientToServer;
        => super::StepColor(box);
        fields {
            step_x: i16,
            step_y: i16,
            transition_time: Uint16,
            options: Options,
        }

        getters {
            /// Return the step in the X color component.
            #[must_use]
            pub const fn step_x(&self) -> i16 {
                self.step_x
            }

            /// Return the step in the Y color component.
            #[must_use]
            pub const fn step_y(&self) -> i16 {
                self.step_y
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
