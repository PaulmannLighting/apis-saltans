use le_stream::FromLeStream;
use zigbee::{DirectedCommand, Direction};

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
use crate::ParseFrameError;

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
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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

impl Command {
    pub fn from_le_stream<T>(
        command_id: u8,
        direction: Direction,
        bytes: T,
    ) -> Result<Self, ParseFrameError>
    where
        T: Iterator<Item = u8>,
    {
        match (command_id, direction) {
            AddGroup::ID => AddGroup::from_le_stream(bytes)
                .map(Self::AddGroup)
                .ok_or(ParseFrameError::InsufficientPayload),
            ViewGroup::ID => ViewGroup::from_le_stream(bytes)
                .map(Self::ViewGroup)
                .ok_or(ParseFrameError::InsufficientPayload),
            GetGroupMembership::ID => GetGroupMembership::from_le_stream(bytes)
                .map(Self::GetGroupMembership)
                .ok_or(ParseFrameError::InsufficientPayload),
            RemoveGroup::ID => RemoveGroup::from_le_stream(bytes)
                .map(Self::RemoveGroup)
                .ok_or(ParseFrameError::InsufficientPayload),
            RemoveAllGroups::ID => RemoveAllGroups::from_le_stream(bytes)
                .map(Self::RemoveAllGroups)
                .ok_or(ParseFrameError::InsufficientPayload),
            AddGroupIfIdentifying::ID => AddGroupIfIdentifying::from_le_stream(bytes)
                .map(Self::AddGroupIfIdentifying)
                .ok_or(ParseFrameError::InsufficientPayload),
            AddGroupResponse::ID => AddGroupResponse::from_le_stream(bytes)
                .map(Self::AddGroupResponse)
                .ok_or(ParseFrameError::InsufficientPayload),
            ViewGroupResponse::ID => ViewGroupResponse::from_le_stream(bytes)
                .map(Self::ViewGroupResponse)
                .ok_or(ParseFrameError::InsufficientPayload),
            GetGroupMembershipResponse::ID => GetGroupMembershipResponse::from_le_stream(bytes)
                .map(Self::GetGroupMembershipResponse)
                .ok_or(ParseFrameError::InsufficientPayload),
            RemoveGroupResponse::ID => RemoveGroupResponse::from_le_stream(bytes)
                .map(Self::RemoveGroupResponse)
                .ok_or(ParseFrameError::InsufficientPayload),
            (other, _) => Err(ParseFrameError::InvalidCommandId(other)),
        }
    }
}
