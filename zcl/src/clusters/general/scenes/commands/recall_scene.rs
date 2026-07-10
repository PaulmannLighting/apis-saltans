use zb_core::types::{Uint8, Uint16};
use zb_core::{Cluster, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Represents a `Recall Scene` command.
    RecallScene {
        { Cluster::Scenes } => Scenes;
        command_id: 0x05;
        direction: Direction::ClientToServer;
        fields {
            group_id: Uint16,
            scene_id: Uint8,
            transition_time: Uint16,
        }
    }
}
