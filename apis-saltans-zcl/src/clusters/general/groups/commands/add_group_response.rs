use apis_saltans_core::types::Uint16;
use apis_saltans_core::{ClusterId, Direction};

use crate::Status;
use crate::macros::zcl_command;

zcl_command! {
    /// Represents a response to an `AddGroups` command.
    AddGroupResponse {
        { ClusterId::Groups } => Groups;
        command_id: 0x00;
        direction: Direction::ServerToClient;
        disable_default_response: true;
        => super::AddGroupResponse(box);
        derive(Copy, Ord, PartialOrd);
        fields {
            status: u8,
            group_id: Uint16,
        }

        constructor {
            /// Creates a new `AddGroupsResponse` with the given status and group ID.
            #[must_use]
            pub fn new(status: Status, group_id: Uint16) -> Self {
                Self {
                    status: status.into(),
                    group_id,
                }
            }
        }

        getters {
            /// Returns the status of the response.
            ///
            /// # Errors
            ///
            /// If the status byte does not correspond to a valid `Status`, this will return the raw status value as an error.
            pub fn status(self) -> Result<Status, u8> {
                Status::try_from(self.status)
            }

            /// Returns the group ID associated with the response.
            #[must_use]
            pub const fn group_id(self) -> Uint16 {
                self.group_id
            }
        }
    }
}
