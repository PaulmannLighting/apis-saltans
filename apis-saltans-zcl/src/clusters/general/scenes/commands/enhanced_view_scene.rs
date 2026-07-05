use apis_saltans_core::types::{Uint8, Uint16};
use apis_saltans_core::{ClusterId, Direction};

use super::EnhancedViewSceneResponse;
use crate::macros::zcl_command;

zcl_command! {
    /// Represents an `Enhanced View Scene` command.
    EnhancedViewScene {
        { ClusterId::Scenes } => Scenes;
        command_id: 0x41;
        direction: Direction::ClientToServer;
        response: EnhancedViewSceneResponse;
        fields {
            group_id: Uint16,
            scene_id: Uint8,
        }
    }
}
