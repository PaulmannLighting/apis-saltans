use apis_saltans_core::types::{String, Uint8, Uint16};
use apis_saltans_core::{ClusterId, Direction};

use super::EnhancedAddSceneResponse;
use crate::clusters::general::scenes::types::ExtensionFieldSets;
use crate::macros::zcl_command;

zcl_command! {
    /// Represents an `Enhanced Add Scene` command.
    EnhancedAddScene {
        { ClusterId::Scenes } => Scenes;
        command_id: 0x40;
        direction: Direction::ClientToServer;
        response: EnhancedAddSceneResponse;
        fields {
            group_id: Uint16,
            scene_id: Uint8,
            transition_time: Uint16,
            scene_name: String<16>,
            extension_field_sets: ExtensionFieldSets,
        }
    }
}
