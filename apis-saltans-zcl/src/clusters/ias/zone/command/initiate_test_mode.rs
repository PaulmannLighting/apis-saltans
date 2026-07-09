use apis_saltans_core::types::Uint8;
use apis_saltans_core::{Cluster, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Initiate test mode command.
    InitiateTestMode {
        { Cluster::IasZone } => IasZone;
        command_id: 0x02;
        direction: Direction::ClientToServer;
        fields {
            test_mode_duration: Uint8,
            current_zone_sensitivity_level: Uint8,
        }
    }
}
