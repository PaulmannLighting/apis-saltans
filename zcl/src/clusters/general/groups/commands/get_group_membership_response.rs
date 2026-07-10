use apis_saltans_core::types::{Uint8, Uint16};
use apis_saltans_core::{Cluster, Direction};

use crate::clusters::general::groups::types::GroupList;
use crate::macros::zcl_command;

zcl_command! {
    /// Represents a response to an `GetGroupMembership` command.
    GetGroupMembershipResponse {
        { Cluster::Groups } => Groups;
        command_id: 0x02;
        direction: Direction::ServerToClient;
        disable_default_response: true;
        fields {
            capacity: Uint8,
            groups: GroupList,
        }

        getters {
            /// Return the remaining capacity of the group table.
            #[must_use]
            pub const fn capacity(&self) -> Uint8 {
                self.capacity
            }

            /// Return the groups in the group table.
            #[must_use]
            pub fn groups(&self) -> &[Uint16] {
                &self.groups
            }
        }
    }
}
