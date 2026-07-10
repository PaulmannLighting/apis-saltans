use apis_saltans_core::{Cluster, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Switch a device on and recall its settings of before it was switched off.
    OnWithRecallGlobalScene {
        { Cluster::OnOff } => OnOff;
        command_id: 0x41;
        direction: Direction::ClientToServer;
        fields;
    }
}
