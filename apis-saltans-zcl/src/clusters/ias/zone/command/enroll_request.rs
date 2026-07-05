use apis_saltans_core::types::Uint16;
use apis_saltans_core::{ClusterId, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Zone enroll request command.
    EnrollRequest {
        { ClusterId::IasZone } => IasZone;
        command_id: 0x01;
        direction: Direction::ServerToClient;
        fields {
            zone_type: Uint16,
            manufacturer_code: Uint16,
        }
    }
}
