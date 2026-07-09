use core::time::Duration;

use apis_saltans_core::types::Uint16;
use apis_saltans_core::{Cluster, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Toggle the identify state of a device.
    Identify {
        { Cluster::Identify } => Identify;
        command_id: 0x00;
        direction: Direction::ClientToServer;
        derive(Copy);
        fields {
            identify_time_secs: Uint16,
        }

        getters {
            /// Return the identify time seconds for this command.
            #[must_use]
            pub fn identify_time_secs(self) -> Option<u16> {
                self.identify_time_secs.into()
            }

            /// Return the identify time for this command.
            #[must_use]
            pub fn identify_time(self) -> Option<Duration> {
                self.identify_time_secs()
                    .map(u64::from)
                    .map(Duration::from_secs)
            }
        }
    }
}
