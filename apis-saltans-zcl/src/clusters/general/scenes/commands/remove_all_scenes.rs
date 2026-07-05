use apis_saltans_core::types::Uint16;
use apis_saltans_core::{ClusterId, Direction};

use super::RemoveAllScenesResponse;
use crate::macros::zcl_command;

zcl_command! {
    /// Represents a `Remove All Scenes` command.
    RemoveAllScenes {
        { ClusterId::Scenes } => Scenes;
        command_id: 0x03;
        direction: Direction::ClientToServer;
        response: RemoveAllScenesResponse;
        fields {
            group_id: Uint16,
        }
    }
}
