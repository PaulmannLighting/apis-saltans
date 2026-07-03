use apis_saltans_core::{ClusterId, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Response to a [`GetAlarm`](super::GetAlarm) command.
    GetAlarmResponse {
        { ClusterId::Alarms } => Alarms;
        command_id: 0x01;
        direction: Direction::ServerToClient;
        disable_default_response: true;
        => super::GetAlarmResponse;
        derive(Copy);
        fields {
            status: u8,
            alarm_code: u8,
            cluster_id: u16,
            timestamp: u32,
        }

        getters {
            /// Returns the status of the `GetAlarm` command.
            #[must_use]
            pub const fn status(self) -> u8 {
                self.status
            }

            /// Returns the alarm code of the earliest generated entry.
            #[must_use]
            pub const fn alarm_code(self) -> u8 {
                self.alarm_code
            }

            /// Returns the cluster ID associated with the earliest generated entry.
            #[must_use]
            pub const fn cluster_id(self) -> u16 {
                self.cluster_id
            }

            /// Returns the timestamp of when the earliest generated entry was created.
            #[must_use]
            pub const fn timestamp(self) -> u32 {
                // TODO: Is this really a `Uint32` or actually a `UtcTime`?
                self.timestamp
            }
        }
    }
}
