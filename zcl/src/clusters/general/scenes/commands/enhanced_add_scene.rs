use zb_core::types::{String, Uint8, Uint16};
use zb_core::{Cluster, Direction};

use super::EnhancedAddSceneResponse;
use crate::clusters::general::scenes::types::ExtensionFieldSets;
use crate::macros::zcl_command;

zcl_command! {
    /// Represents an `Enhanced Add Scene` command.
    EnhancedAddScene {
        { Cluster::Scenes } => Scenes;
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
