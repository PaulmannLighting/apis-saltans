use apis_saltans_core::{ClusterId, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Switch a device on.
    On {
        { ClusterId::OnOff } => OnOff;
        command_id: 0x01;
        direction: Direction::ClientToServer;
        => super::On(box);
        fields;
    }
}
