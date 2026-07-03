use apis_saltans_core::{ClusterId, Direction};

use crate::Options;
use crate::macros::zcl_command;

zcl_command! {
    /// Command to stop a move step in a lighting device.
    StopMoveStep {
        { ClusterId::ColorControl } => ColorControl;
        command_id: 47;
        direction: Direction::ClientToServer;
        => super::StopMoveStep;
        derive(Copy);
        fields {
            options: Options,
        }

        getters {
            /// Return the options for the command.
            #[must_use]
            pub const fn options(self) -> Options {
                self.options
            }
        }
    }
}
