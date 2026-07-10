use apis_saltans_core::{Cluster, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Command to remove all groups from the device's group table.
    RemoveAllGroups {
        { Cluster::Groups } => Groups;
        command_id: 0x04;
        direction: Direction::ClientToServer;
        fields;
    }
}
