use apis_saltans_core::{ClusterId, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Toggle a device on/off state.
    Toggle {
        { ClusterId::OnOff } => OnOff;
        command_id: 0x02;
        direction: Direction::ClientToServer;
        => super::Toggle(box);
        fields;
    }
}
