use apis_saltans_core::types::{String, Uint16};
use apis_saltans_core::{ClusterId, Direction};

use crate::Status;
use crate::macros::zcl_command;

zcl_command! {
    /// Represents a response to an `ViewGroup` command.
    ViewGroupResponse {
        { ClusterId::Groups } => Groups;
        command_id: 0x01;
        direction: Direction::ServerToClient;
        disable_default_response: true;
        fields {
            status: u8,
            group_id: Uint16,
            group_name: String,
        }

        constructor {
            /// Creates a new `ViewGroupResponse` with the given status and group ID.
            #[must_use]
            pub fn new(status: Status, group_id: Uint16, group_name: String) -> Self {
                Self {
                    status: status.into(),
                    group_id,
                    group_name,
                }
            }
        }

        getters {
            /// Returns the status of the response.
            ///
            /// # Errors
            ///
            /// If the status byte does not correspond to a valid `Status`, this will return the raw status value as an error.
            pub fn status(&self) -> Result<Status, u8> {
                Status::try_from(self.status)
            }

            /// Returns the group ID associated with the response.
            #[must_use]
            pub const fn group_id(&self) -> Uint16 {
                self.group_id
            }

            /// Returns the name of the group associated with the response.
            #[must_use]
            pub const fn group_name(&self) -> &String {
                &self.group_name
            }
        }
    }
}
