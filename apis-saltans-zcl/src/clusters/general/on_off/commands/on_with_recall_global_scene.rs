use apis_saltans_core::{ClusterId, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Switch a device on and recall its settings of before it was switched off.
    OnWithRecallGlobalScene {
        { ClusterId::OnOff } => OnOff;
        command_id: 0x41;
        direction: Direction::ClientToServer;
        => super::OnWithRecallGlobalScene;
        fields;
    }
}
