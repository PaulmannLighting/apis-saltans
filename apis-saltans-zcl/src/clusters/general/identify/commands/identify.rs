use core::time::Duration;

use apis_saltans_core::types::Uint16;
use apis_saltans_core::{ClusterId, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Toggle the identify state of a device.
    Identify {
        { ClusterId::Identify } => Identify;
        command_id: 0x00;
        direction: Direction::ClientToServer;
        => super::Identify;
        derive(Copy, Ord, PartialOrd);
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
