use apis_saltans_core::types::{String, Uint8, Uint16};
use apis_saltans_core::{ClusterId, Direction};

use super::AddSceneResponse;
use crate::clusters::general::scenes::types::ExtensionFieldSets;
use crate::macros::zcl_command;

zcl_command! {
    /// Represents an `Add Scene` command.
    AddScene {
        { ClusterId::Scenes } => Scenes;
        command_id: 0x00;
        direction: Direction::ClientToServer;
        response: AddSceneResponse;
        fields {
            group_id: Uint16,
            scene_id: Uint8,
            transition_time: Uint16,
            scene_name: String<16>,
            extension_field_sets: ExtensionFieldSets,
        }
    }
}
