use apis_saltans_core::{Cluster, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Switch a device off.
    Off {
        { Cluster::OnOff } => OnOff;
        command_id: 0x00;
        direction: Direction::ClientToServer;
        fields;
    }
}
