use apis_saltans_core::{Cluster, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Initiate normal operation mode command.
    InitiateNormalOperationMode {
        { Cluster::IasZone } => IasZone;
        command_id: 0x01;
        direction: Direction::ClientToServer;
        fields;
    }
}
