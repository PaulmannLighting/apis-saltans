use apis_saltans_core::types::Uint16;
use apis_saltans_core::{ClusterId, Direction};

use super::GetSceneMembershipResponse;
use crate::macros::zcl_command;

zcl_command! {
    /// Represents a `Get Scene Membership` command.
    GetSceneMembership {
        { ClusterId::Scenes } => Scenes;
        command_id: 0x06;
        direction: Direction::ClientToServer;
        response: GetSceneMembershipResponse;
        fields {
            group_id: Uint16,
        }
    }
}
