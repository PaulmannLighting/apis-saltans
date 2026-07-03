use core::time::Duration;

use apis_saltans_core::types::Uint16;
use apis_saltans_core::{ClusterId, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Response to the [`IdentifyQuery`](crate::clusters::general::identify::IdentifyQuery) command.
    IdentifyQueryResponse {
        { ClusterId::Identify } => Identify;
        command_id: 0x00;
        direction: Direction::ServerToClient;
        disable_default_response: true;
        => super::IdentifyQueryResponse;
        derive(Copy);
        fields {
            timeout_secs: Uint16,
        }

        getters {
            /// Return the identify time in seconds.
            #[must_use]
            pub fn timeout_secs(self) -> Option<u16> {
                self.timeout_secs.into()
            }

            /// Return the identify timeout for this command.
            #[must_use]
            pub fn timeout(self) -> Option<Duration> {
                self.timeout_secs().map(u64::from).map(Duration::from_secs)
            }
        }
    }
}
