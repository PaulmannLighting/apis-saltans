use zb_core::types::{Uint8, Uint16};
use zb_core::{Cluster, Direction};

use crate::Status;
use crate::macros::zcl_command;

zcl_command! {
    /// Represents a `Remove Scene Response` command.
    RemoveSceneResponse {
        { Cluster::Scenes } => Scenes;
        command_id: 0x02;
        direction: Direction::ServerToClient;
        disable_default_response: true;
        fields {
            status: u8,
            group_id: Uint16,
            scene_id: Uint8,
        }

        constructor {
            /// Creates a new command payload.
            #[must_use]
            pub fn new(status: Status, group_id: Uint16, scene_id: Uint8) -> Self {
                Self { status: status.into(), group_id, scene_id }
            }
        }

        getters {
            /// Return the status of the response.
            ///
            /// # Errors
            ///
            /// Returns the raw status code if the conversion to a [`Status`] fails.
            pub fn status(&self) -> Result<Status, u8> {
                self.status.try_into()
            }
        }
    }
}
