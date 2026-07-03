use apis_saltans_core::types::Uint16;
use apis_saltans_core::{ClusterId, Direction};

use crate::ias::zone::Status;
use crate::macros::zcl_command;

zcl_command! {
    /// Zone status change attributes.
    StatusChange {
        { ClusterId::IasZone } => IasZone;
        command_id: 0x00;
        direction: Direction::ServerToClient;
        => super::StatusChange(box);
        derive(Ord, PartialOrd);
        fields {
            status: Status,
            extended_status: u8,
            zone_id: u8,
            delay: Uint16,
        }

        getters {
            /// Return the status.
            #[must_use]
            pub const fn status(&self) -> Status {
                self.status
            }

            /// Return the extended status.
            #[must_use]
            pub const fn extended_status(&self) -> u8 {
                self.extended_status
            }

            /// Return the zone ID.
            #[must_use]
            pub const fn zone_id(&self) -> u8 {
                self.zone_id
            }
        }
    }
}
