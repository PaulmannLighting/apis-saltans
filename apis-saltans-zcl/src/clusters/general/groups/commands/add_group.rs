use core::str::Utf8Error;

use apis_saltans_core::types::{String, Uint16};
use apis_saltans_core::{ClusterId, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Command to add a group to the device's group table.
    AddGroup {
        { ClusterId::Groups } => Groups;
        command_id: 0x00;
        direction: Direction::ClientToServer;
        => super::AddGroup;
        fields {
            /// The identifier of the group to be added.
            group_id: Uint16,
            /// The name of the group to be added, if supported.
            group_name: String,
        }

        getters {
            /// Returns the identifier of the group to be added.
            #[must_use]
            pub const fn group_id(&self) -> Uint16 {
                self.group_id
            }

            /// Returns the name of the group to be added.
            ///
            /// # Errors
            ///
            /// If the group name is not valid UTF-8, this will return an [`Utf8Error`].
            pub fn group_name(&self) -> Result<&str, Utf8Error> {
                self.group_name.try_as_str()
            }

            /// Returns the raw bytes of the group name.
            #[must_use]
            pub fn group_name_raw(&self) -> &[u8] {
                self.group_name.as_ref()
            }
        }
    }
}
