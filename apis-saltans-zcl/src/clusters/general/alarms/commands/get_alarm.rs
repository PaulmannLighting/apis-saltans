use apis_saltans_core::{Cluster, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Returns the alarm with the earliest generated entry.
    GetAlarm {
        { Cluster::Alarms } => Alarms;
        command_id: 0x02;
        direction: Direction::ClientToServer;
        derive(Default);
        fields;
    }
}
