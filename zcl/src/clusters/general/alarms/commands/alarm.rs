use zb_core::{Cluster, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// An issued alarm.
    Alarm {
        { Cluster::Alarms } => Alarms;
        command_id: 0x00;
        direction: Direction::ServerToClient;
        derive(Copy);
        fields {
            code: u8,
            cluster_id: u16,
        }

        getters {
            /// Returns the alarm code.
            #[must_use]
            pub const fn code(self) -> u8 {
                self.code
            }

            /// Returns the cluster ID associated with the alarm.
            #[must_use]
            pub const fn cluster_id(self) -> u16 {
                self.cluster_id
            }
        }
    }
}
