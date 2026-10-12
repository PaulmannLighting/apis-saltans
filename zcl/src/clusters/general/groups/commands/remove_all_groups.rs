use zb_core::{Cluster, Direction};

use crate::DefaultResponsePolicy;
use crate::macros::zcl_command;

zcl_command! {
    /// Command to remove all groups from the device's group table.
    RemoveAllGroups {
        { Cluster::Groups } => Groups;
        command_id: 0x04;
        direction: Direction::ClientToServer;
        response: crate::NoSpecificResponse;
        default_response_policy: DefaultResponsePolicy::DefaultAllowed;
        fields;
    }
}
