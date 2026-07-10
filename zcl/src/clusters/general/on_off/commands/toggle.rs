use apis_saltans_core::{Cluster, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Toggle a device on/off state.
    Toggle {
        { Cluster::OnOff } => OnOff;
        command_id: 0x02;
        direction: Direction::ClientToServer;
        fields;
    }
}
