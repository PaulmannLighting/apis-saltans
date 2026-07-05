use apis_saltans_core::{ClusterId, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Initiate normal operation mode command.
    InitiateNormalOperationMode {
        { ClusterId::IasZone } => IasZone;
        command_id: 0x01;
        direction: Direction::ClientToServer;
        fields;
    }
}
