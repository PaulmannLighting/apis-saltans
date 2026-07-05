use apis_saltans_core::{ClusterId, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Command to remove all groups from the device's group table.
    RemoveAllGroups {
        { ClusterId::Groups } => Groups;
        command_id: 0x04;
        direction: Direction::ClientToServer;
        fields;
    }
}
