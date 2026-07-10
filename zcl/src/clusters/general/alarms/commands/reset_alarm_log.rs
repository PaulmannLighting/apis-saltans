use zb_core::{Cluster, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Clear the alarm table.
    ResetAlarmLog {
        { Cluster::Alarms } => Alarms;
        command_id: 0x03;
        direction: Direction::ClientToServer;
        derive(Default);
        fields;
    }
}
