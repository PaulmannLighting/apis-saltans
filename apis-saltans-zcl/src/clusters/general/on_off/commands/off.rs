use apis_saltans_core::{ClusterId, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Switch a device off.
    Off {
        { ClusterId::OnOff } => OnOff;
        command_id: 0x00;
        direction: Direction::ClientToServer;
        => super::Off;
        fields;
    }
}
