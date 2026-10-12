use zb_core::{Cluster, Direction};

use crate::global::default_response::DefaultResponse;
use crate::macros::zcl_command;

zcl_command! {
    /// Command to remove all groups from the device's group table.
    RemoveAllGroups {
        { Cluster::Groups } => Groups;
        command_id: 0x04;
        direction: Direction::ClientToServer;
        response: DefaultResponse;
        fields;
    }
}
