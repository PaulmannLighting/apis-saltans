use zb_core::types::{Uint8, Uint16};
use zb_core::{Cluster, Direction};

use crate::Status;
use crate::clusters::general::scenes::types::SceneList;
use crate::macros::zcl_command;

zcl_command! {
    /// Represents a `Get Scene Membership Response` command.
    GetSceneMembershipResponse {
        { Cluster::Scenes } => Scenes;
        command_id: 0x06;
        direction: Direction::ServerToClient;
        disable_default_response: true;
        fields {
            status: u8,
            capacity: Uint8,
            group_id: Uint16,
            scene_list: SceneList,
        }

        constructor {
            /// Creates a new command payload.
            #[must_use]
            pub fn new(status: Status, capacity: Uint8, group_id: Uint16, scene_list: SceneList) -> Self {
                Self { status: status.into(), capacity, group_id, scene_list }
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
