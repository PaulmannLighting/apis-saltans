use apis_saltans_core::types::Uint16;
use apis_saltans_core::{Cluster, Direction};

use crate::macros::zcl_command;

zcl_command! {
    /// Command to remove a group from the device's group table.
    #[repr(transparent)]
    RemoveGroup {
        { Cluster::Groups } => Groups;
        command_id: 0x03;
        direction: Direction::ClientToServer;
        derive(Copy);
        fields {
            group_id: Uint16,
        }

        getters {
            /// Returns the identifier of the group to be removed.
            #[must_use]
            pub const fn group_id(self) -> Uint16 {
                self.group_id
            }
        }
    }
}
