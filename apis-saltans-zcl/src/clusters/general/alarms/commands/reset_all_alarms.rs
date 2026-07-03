use apis_saltans_core::{ClusterId, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Reset all alarms.
    ResetAllAlarms {
        { ClusterId::Alarms } => Alarms;
        command_id: 0x01;
        direction: Direction::ClientToServer;
        => super::ResetAllAlarms(box);
        derive(Default);
        fields;
    }
}
