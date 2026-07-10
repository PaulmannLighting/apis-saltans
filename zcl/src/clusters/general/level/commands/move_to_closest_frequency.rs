use zb_core::{Cluster, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Move to the closest frequency command.
    MoveToClosestFrequency {
        { Cluster::Level } => Level;
        command_id: 0x08;
        direction: Direction::ClientToServer;
        derive(Copy);
        fields {
            frequency: u16,
        }

        getters {
            /// Get the frequency.
            #[must_use]
            pub const fn frequency(self) -> u16 {
                self.frequency
            }
        }
    }
}
