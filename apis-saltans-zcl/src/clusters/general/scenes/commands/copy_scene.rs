use apis_saltans_core::types::{Uint8, Uint16};
use apis_saltans_core::{ClusterId, Direction};

use super::CopySceneResponse;
use crate::macros::zcl_command;

zcl_command! {
    /// Represents a `Copy Scene` command.
    CopyScene {
        { ClusterId::Scenes } => Scenes;
        command_id: 0x42;
        direction: Direction::ClientToServer;
        response: CopySceneResponse;
        fields {
            mode: u8,
            group_id_from: Uint16,
            scene_id_from: Uint8,
            group_id_to: Uint16,
            scene_id_to: Uint8,
        }
    }
}
