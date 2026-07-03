use apis_saltans_core::{ClusterId, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Returns the alarm with the earliest generated entry.
    GetAlarm {
        { ClusterId::Alarms } => Alarms;
        command_id: 0x02;
        direction: Direction::ClientToServer;
        => super::GetAlarm(box);
        derive(Default);
        fields;
    }
}
