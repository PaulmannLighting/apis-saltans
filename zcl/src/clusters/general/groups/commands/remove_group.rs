use zb_core::types::Uint16;
use zb_core::{Cluster, Direction, GroupId};

use super::RemoveGroupResponse;
use crate::macros::zcl_command;

zcl_command! {
    /// Command to remove a group from the device's group table.
    #[repr(transparent)]
    RemoveGroup {
        { Cluster::Groups } => Groups;
        command_id: 0x03;
        direction: Direction::ClientToServer;
        response: RemoveGroupResponse;
        derive(Copy);
        fields {
            group_id: Uint16,
        }

        constructor {
            /// Creates a new command payload.
            #[must_use]
            pub const fn new(group_id: GroupId) -> Self {
                Self {
                    group_id: Uint16::new(group_id.as_u16()),
                }
            }
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
