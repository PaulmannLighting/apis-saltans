use zb_core::types::Uint16;
use zb_core::{Cluster, Direction};

use super::GetSceneMembershipResponse;
use crate::macros::zcl_command;

zcl_command! {
    /// Represents a `Get Scene Membership` command.
    GetSceneMembership {
        { Cluster::Scenes } => Scenes;
        command_id: 0x06;
        direction: Direction::ClientToServer;
        response: GetSceneMembershipResponse;
        fields {
            group_id: Uint16,
        }
    }
}
