use zb_core::types::{Uint8, Uint16};
use zb_core::{Cluster, Direction};

use super::ViewSceneResponse;
use crate::macros::zcl_command;

zcl_command! {
    /// Represents a `View Scene` command.
    ViewScene {
        { Cluster::Scenes } => Scenes;
        command_id: 0x01;
        direction: Direction::ClientToServer;
        response: ViewSceneResponse;
        fields {
            group_id: Uint16,
            scene_id: Uint8,
        }
    }
}
