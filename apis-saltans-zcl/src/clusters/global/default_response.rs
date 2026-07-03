//! Default Response Command.

use apis_saltans_core::Direction;

use crate::macros::zcl_command;

zcl_command! {
    /// Default Response Command
    DefaultResponse {
        Global;
        command_id: 0x0b;
        direction: Direction::ClientToServer;
        disable_default_response: true;
        => crate::global::DefaultResponse(box);
        fields {
            status: u8,
            command_id: u8,
        }

        getters {
            /// Return the status of the default response.
            #[must_use]
            pub const fn status(&self) -> u8 {
                self.status
            }

            /// Return the command ID of the default response.
            #[must_use]
            pub const fn command_id(&self) -> u8 {
                self.command_id
            }
        }
    }
}
