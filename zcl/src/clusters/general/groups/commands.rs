use zigbee::Cluster;
use zigbee_macros::ParseZclFrame;

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

/// Available Groups cluster commands.
#[derive(Clone, Debug, Eq, PartialEq, Hash, ParseZclFrame)]
pub enum Command {
    /// Add Group command.
    AddGroup(AddGroup),
    /// View Group command.
    ViewGroup(ViewGroup),
    /// Get Group Membership command.
    GetGroupMembership(GetGroupMembership),
    /// Remove Group command.
    RemoveGroup(RemoveGroup),
    /// Remove All Groups command.
    RemoveAllGroups(RemoveAllGroups),
    /// Add Group If Identifying command.
    AddGroupIfIdentifying(AddGroupIfIdentifying),
    /// Add Group Response command.
    AddGroupResponse(AddGroupResponse),
    /// View Group Response command.
    ViewGroupResponse(ViewGroupResponse),
    /// Get Group Membership Response command.
    GetGroupMembershipResponse(GetGroupMembershipResponse),
    /// Remove Group Response command.
    RemoveGroupResponse(RemoveGroupResponse),
}

impl Cluster for Command {
    const ID: u16 = super::CLUSTER_ID;
}
