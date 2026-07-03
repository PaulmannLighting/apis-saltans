use apis_saltans_core::types::Uint16;
use apis_saltans_core::{ClusterId, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Command to view a group in the device's group table.
    ViewGroup {
        { ClusterId::Groups } => Groups;
        command_id: 0x01;
        direction: Direction::ClientToServer;
        => super::ViewGroup;
        derive(Copy);
        fields {
            /// The identifier of the group to view.
            group_id: Uint16,
        }

        getters {
            /// Returns the identifier of the group to view.
            #[must_use]
            pub const fn group_id(self) -> Uint16 {
                self.group_id
            }
        }
    }
}
