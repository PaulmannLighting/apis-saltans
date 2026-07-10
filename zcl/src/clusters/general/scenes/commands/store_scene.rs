use apis_saltans_core::types::{Uint8, Uint16};
use apis_saltans_core::{Cluster, Direction};

use super::StoreSceneResponse;
use crate::macros::zcl_command;

zcl_command! {
    /// Represents a `Store Scene` command.
    StoreScene {
        { Cluster::Scenes } => Scenes;
        command_id: 0x04;
        direction: Direction::ClientToServer;
        response: StoreSceneResponse;
        fields {
            group_id: Uint16,
            scene_id: Uint8,
        }
    }
}
