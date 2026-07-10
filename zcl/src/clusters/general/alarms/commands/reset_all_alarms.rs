use zb_core::{Cluster, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Reset all alarms.
    ResetAllAlarms {
        { Cluster::Alarms } => Alarms;
        command_id: 0x01;
        direction: Direction::ClientToServer;
        derive(Default);
        fields;
    }
}
