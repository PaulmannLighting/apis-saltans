use apis_saltans_core::types::Uint8;
use apis_saltans_core::{ClusterId, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Zone enroll response command.
    EnrollResponse {
        { ClusterId::IasZone } => IasZone;
        command_id: 0x00;
        direction: Direction::ClientToServer;
        fields {
            response_code: u8,
            zone_id: Uint8,
        }
    }
}
