use apis_saltans_core::types::{Uint8, Uint16};
use apis_saltans_core::{ClusterId, Direction};

use super::RemoveSceneResponse;
use crate::macros::zcl_command;

zcl_command! {
    /// Represents a `Remove Scene` command.
    RemoveScene {
        { ClusterId::Scenes } => Scenes;
        command_id: 0x02;
        direction: Direction::ClientToServer;
        response: RemoveSceneResponse;
        fields {
            group_id: Uint16,
            scene_id: Uint8,
        }
    }
}
