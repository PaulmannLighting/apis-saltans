use zb_core::types::{String, Uint8, Uint16};
use zb_core::{Cluster, Direction};

use crate::Status;
use crate::clusters::general::scenes::types::ExtensionFieldSets;
use crate::macros::zcl_command;

zcl_command! {
    /// Represents an `Enhanced View Scene Response` command.
    EnhancedViewSceneResponse {
        { Cluster::Scenes } => Scenes;
        command_id: 0x41;
        direction: Direction::ServerToClient;
        disable_default_response: true;
        fields {
            status: u8,
            group_id: Uint16,
            scene_id: Uint8,
            transition_time: Uint16,
            scene_name: String<16>,
            extension_field_sets: ExtensionFieldSets,
        }

        constructor {
            /// Creates a new command payload.
            #[must_use]
            pub fn new(
                status: Status,
                group_id: Uint16,
                scene_id: Uint8,
                transition_time: Uint16,
                scene_name: String<16>,
                extension_field_sets: ExtensionFieldSets,
            ) -> Self {
                Self {
                    status: status.into(),
                    group_id,
                    scene_id,
                    transition_time,
                    scene_name,
                    extension_field_sets,
                }
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
