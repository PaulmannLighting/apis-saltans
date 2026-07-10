//! Discover Commands Generated Command and Response.

use std::boxed::Box;

use zb_core::Direction;

use crate::macros::zcl_command;

zcl_command! {
    /// Discover Commands Generated Command.
    Command {
        Global;
        command_id: 0x13;
        direction: Direction::ClientToServer;
        response: Response;
        => crate::global::DiscoverCommandsGenerated;
        fields {
            start_command_id: u8,
            maximum_command_ids: u8,
        }
    }
}

zcl_command! {
    /// Discover Commands Generated Response Command.
    Response {
        Global;
        command_id: 0x14;
        direction: Direction::ServerToClient;
        => crate::global::DiscoverCommandsGeneratedResponse;
        fields {
            discovery_complete: u8,
            command_ids: Box<[u8]>,
        }

        getters {
            /// Return whether command discovery is complete.
            #[must_use]
            pub const fn discovery_complete(&self) -> u8 {
                self.discovery_complete
            }

            /// Return the discovered command identifiers.
            #[must_use]
            pub fn command_ids(&self) -> &[u8] {
                &self.command_ids
            }
        }
    }
}
