use apis_saltans_core::types::Uint16;
use apis_saltans_core::{ClusterId, Direction};

use crate::general::groups::types::GroupList;
use crate::macros::zcl_command;

zcl_command! {
    /// Command to request the membership of a device in multiple groups.
    GetGroupMembership {
        { ClusterId::Groups } => Groups;
        command_id: 0x02;
        direction: Direction::ClientToServer;
        fields {
            groups: GroupList,
        }

        getters {
            /// Return the groups the sender is a member of.
            #[must_use]
            pub fn groups(&self) -> &[Uint16] {
                &self.groups
            }
        }
    }
}
