use apis_saltans_core::types::{String, Uint16};
use apis_saltans_core::{ClusterId, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Command to add a group to the device's group table if the device is currently identifying.
    AddGroupIfIdentifying {
        { ClusterId::Groups } => Groups;
        command_id: 0x05;
        direction: Direction::ClientToServer;
        => super::AddGroupIfIdentifying;
        fields {
            group_id: Uint16,
            group_name: String,
        }

        getters {
            /// Returns the identifier of the group to be added.
            #[must_use]
            pub const fn group_id(&self) -> Uint16 {
                self.group_id
            }

            /// Returns the name of the group to be added.
            #[must_use]
            pub const fn group_name(&self) -> &String {
                &self.group_name
            }
        }
    }
}
