use zb_core::types::Uint16;
use zb_core::{Cluster, Direction};

use super::GetGroupMembershipResponse;
use crate::groups::types::GroupList;
use crate::macros::zcl_command;

zcl_command! {
    /// Command to request the membership of a device in multiple groups.
    GetGroupMembership {
        { Cluster::Groups } => Groups;
        command_id: 0x02;
        direction: Direction::ClientToServer;
        response: GetGroupMembershipResponse;
        derive(Default);
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
