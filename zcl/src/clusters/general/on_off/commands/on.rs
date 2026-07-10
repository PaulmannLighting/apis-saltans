use apis_saltans_core::{Cluster, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Switch a device on.
    On {
        { Cluster::OnOff } => OnOff;
        command_id: 0x01;
        direction: Direction::ClientToServer;
        fields;
    }
}
