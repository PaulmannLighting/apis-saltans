use zb_core::types::Uint16;
use zb_core::{Cluster, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Zone enroll request command.
    EnrollRequest {
        { Cluster::IasZone } => IasZone;
        command_id: 0x01;
        direction: Direction::ServerToClient;
        fields {
            zone_type: Uint16,
            manufacturer_code: Uint16,
        }
    }
}
