use apis_saltans_core::{ClusterId, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Clear the alarm table.
    ResetAlarmLog {
        { ClusterId::Alarms } => Alarms;
        command_id: 0x03;
        direction: Direction::ClientToServer;
        => super::ResetAlarmLog;
        derive(Default);
        fields;
    }
}
