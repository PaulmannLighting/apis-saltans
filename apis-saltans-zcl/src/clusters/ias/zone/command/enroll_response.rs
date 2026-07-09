use apis_saltans_core::types::Uint8;
use apis_saltans_core::{Cluster, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Zone enroll response command.
    EnrollResponse {
        { Cluster::IasZone } => IasZone;
        command_id: 0x00;
        direction: Direction::ClientToServer;
        fields {
            response_code: u8,
            zone_id: Uint8,
        }
    }
}
