use zb_core::Cluster;

pub use self::add_group::AddGroup;
pub use self::add_group_if_identifying::AddGroupIfIdentifying;
pub use self::add_group_response::AddGroupResponse;
pub use self::get_group_membership::GetGroupMembership;
pub use self::get_group_membership_response::GetGroupMembershipResponse;
pub use self::remove_all_groups::RemoveAllGroups;
pub use self::remove_group::RemoveGroup;
pub use self::remove_group_response::RemoveGroupResponse;
pub use self::view_group::ViewGroup;
pub use self::view_group_response::ViewGroupResponse;
use crate::macros::zcl_command_enum;

mod add_group;
mod add_group_if_identifying;
mod add_group_response;
mod get_group_membership;
mod get_group_membership_response;
mod remove_all_groups;
mod remove_group;
mod remove_group_response;
mod view_group;
mod view_group_response;

// Available Groups cluster commands.
zcl_command_enum! {
    { Cluster::Groups } => Groups;
    AddGroup(AddGroup),
    ViewGroup(ViewGroup),
    GetGroupMembership(GetGroupMembership),
    RemoveGroup(RemoveGroup),
    RemoveAllGroups(RemoveAllGroups),
    AddGroupIfIdentifying(AddGroupIfIdentifying),
    AddGroupResponse(AddGroupResponse),
    ViewGroupResponse(ViewGroupResponse),
    GetGroupMembershipResponse(GetGroupMembershipResponse),
    RemoveGroupResponse(RemoveGroupResponse),
}
