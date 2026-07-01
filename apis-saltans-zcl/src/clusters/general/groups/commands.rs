use le_stream::ToLeStream;
use apis_saltans_core::{ClusterId, ClusterSpecific, Direction};
use apis_saltans_macros::ParseZclFrame;

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
use crate::{CommandDispatch, Scope};

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
    AddGroup(Box<AddGroup>),

    /// View Group command.
    ViewGroup(ViewGroup),

    /// Get Group Membership command.
    GetGroupMembership(GetGroupMembership),

    /// Remove Group command.
    RemoveGroup(RemoveGroup),

    /// Remove All Groups command.
    RemoveAllGroups(RemoveAllGroups),

    /// Add a Group If Identifying command.
    AddGroupIfIdentifying(Box<AddGroupIfIdentifying>),

    /// Add Group Response command.
    AddGroupResponse(AddGroupResponse),

    /// View Group Response command.
    ViewGroupResponse(Box<ViewGroupResponse>),

    /// Get Group Membership Response command.
    GetGroupMembershipResponse(Box<GetGroupMembershipResponse>),

    /// Remove Group Response command.
    RemoveGroupResponse(RemoveGroupResponse),
}

impl ClusterSpecific for Command {
    const CLUSTER: ClusterId = ClusterId::Groups;
}

impl From<Command> for crate::Cluster {
    fn from(command: Command) -> Self {
        Self::Groups(command)
    }
}

impl From<AddGroup> for Command {
    fn from(command: AddGroup) -> Self {
        Self::AddGroup(command.into())
    }
}

impl From<ViewGroup> for Command {
    fn from(command: ViewGroup) -> Self {
        Self::ViewGroup(command)
    }
}

impl From<GetGroupMembership> for Command {
    fn from(command: GetGroupMembership) -> Self {
        Self::GetGroupMembership(command)
    }
}

impl From<RemoveGroup> for Command {
    fn from(command: RemoveGroup) -> Self {
        Self::RemoveGroup(command)
    }
}

impl From<RemoveAllGroups> for Command {
    fn from(command: RemoveAllGroups) -> Self {
        Self::RemoveAllGroups(command)
    }
}

impl From<AddGroupIfIdentifying> for Command {
    fn from(command: AddGroupIfIdentifying) -> Self {
        Self::AddGroupIfIdentifying(command.into())
    }
}

impl From<AddGroupResponse> for Command {
    fn from(response: AddGroupResponse) -> Self {
        Self::AddGroupResponse(response)
    }
}

impl From<ViewGroupResponse> for Command {
    fn from(response: ViewGroupResponse) -> Self {
        Self::ViewGroupResponse(response.into())
    }
}

impl From<GetGroupMembershipResponse> for Command {
    fn from(response: GetGroupMembershipResponse) -> Self {
        Self::GetGroupMembershipResponse(response.into())
    }
}

impl From<RemoveGroupResponse> for Command {
    fn from(response: RemoveGroupResponse) -> Self {
        Self::RemoveGroupResponse(response)
    }
}

impl CommandDispatch for Command {
    fn command_id(&self) -> u8 {
        match self {
            Self::AddGroup(cmd) => cmd.command_id(),
            Self::ViewGroup(cmd) => cmd.command_id(),
            Self::GetGroupMembership(cmd) => cmd.command_id(),
            Self::RemoveGroup(cmd) => cmd.command_id(),
            Self::RemoveAllGroups(cmd) => cmd.command_id(),
            Self::AddGroupIfIdentifying(cmd) => cmd.command_id(),
            Self::AddGroupResponse(cmd) => cmd.command_id(),
            Self::ViewGroupResponse(cmd) => cmd.command_id(),
            Self::GetGroupMembershipResponse(cmd) => cmd.command_id(),
            Self::RemoveGroupResponse(cmd) => cmd.command_id(),
        }
    }

    fn scope(&self) -> Scope {
        match self {
            Self::AddGroup(cmd) => cmd.scope(),
            Self::ViewGroup(cmd) => cmd.scope(),
            Self::GetGroupMembership(cmd) => cmd.scope(),
            Self::RemoveGroup(cmd) => cmd.scope(),
            Self::RemoveAllGroups(cmd) => cmd.scope(),
            Self::AddGroupIfIdentifying(cmd) => cmd.scope(),
            Self::AddGroupResponse(cmd) => cmd.scope(),
            Self::ViewGroupResponse(cmd) => cmd.scope(),
            Self::GetGroupMembershipResponse(cmd) => cmd.scope(),
            Self::RemoveGroupResponse(cmd) => cmd.scope(),
        }
    }

    fn direction(&self) -> Direction {
        match self {
            Self::AddGroup(cmd) => cmd.direction(),
            Self::ViewGroup(cmd) => cmd.direction(),
            Self::GetGroupMembership(cmd) => cmd.direction(),
            Self::RemoveGroup(cmd) => cmd.direction(),
            Self::RemoveAllGroups(cmd) => cmd.direction(),
            Self::AddGroupIfIdentifying(cmd) => cmd.direction(),
            Self::AddGroupResponse(cmd) => cmd.direction(),
            Self::ViewGroupResponse(cmd) => cmd.direction(),
            Self::GetGroupMembershipResponse(cmd) => cmd.direction(),
            Self::RemoveGroupResponse(cmd) => cmd.direction(),
        }
    }

    fn disable_default_response(&self) -> bool {
        match self {
            Self::AddGroup(cmd) => cmd.disable_default_response(),
            Self::ViewGroup(cmd) => cmd.disable_default_response(),
            Self::GetGroupMembership(cmd) => cmd.disable_default_response(),
            Self::RemoveGroup(cmd) => cmd.disable_default_response(),
            Self::RemoveAllGroups(cmd) => cmd.disable_default_response(),
            Self::AddGroupIfIdentifying(cmd) => cmd.disable_default_response(),
            Self::AddGroupResponse(cmd) => cmd.disable_default_response(),
            Self::ViewGroupResponse(cmd) => cmd.disable_default_response(),
            Self::GetGroupMembershipResponse(cmd) => cmd.disable_default_response(),
            Self::RemoveGroupResponse(cmd) => cmd.disable_default_response(),
        }
    }
}

impl ToLeStream for Command {
    type Iter = Iter;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::AddGroup(cmd) => Iter::AddGroup(cmd.to_le_stream()),
            Self::ViewGroup(cmd) => Iter::ViewGroup(cmd.to_le_stream()),
            Self::GetGroupMembership(cmd) => Iter::GetGroupMembership(cmd.to_le_stream()),
            Self::RemoveGroup(cmd) => Iter::RemoveGroup(cmd.to_le_stream()),
            Self::RemoveAllGroups(cmd) => Iter::RemoveAllGroups(cmd.to_le_stream()),
            Self::AddGroupIfIdentifying(cmd) => Iter::AddGroupIfIdentifying(cmd.to_le_stream()),
            Self::AddGroupResponse(cmd) => Iter::AddGroupResponse(cmd.to_le_stream()),
            Self::ViewGroupResponse(cmd) => Iter::ViewGroupResponse(cmd.to_le_stream().into()),
            Self::GetGroupMembershipResponse(cmd) => {
                Iter::GetGroupMembershipResponse(cmd.to_le_stream().into())
            }
            Self::RemoveGroupResponse(cmd) => Iter::RemoveGroupResponse(cmd.to_le_stream()),
        }
    }
}

#[derive(Debug)]
pub enum Iter {
    AddGroup(<AddGroup as ToLeStream>::Iter),
    ViewGroup(<ViewGroup as ToLeStream>::Iter),
    GetGroupMembership(<GetGroupMembership as ToLeStream>::Iter),
    RemoveGroup(<RemoveGroup as ToLeStream>::Iter),
    RemoveAllGroups(<RemoveAllGroups as ToLeStream>::Iter),
    AddGroupIfIdentifying(<AddGroupIfIdentifying as ToLeStream>::Iter),
    AddGroupResponse(<AddGroupResponse as ToLeStream>::Iter),
    ViewGroupResponse(Box<<ViewGroupResponse as ToLeStream>::Iter>),
    GetGroupMembershipResponse(Box<<GetGroupMembershipResponse as ToLeStream>::Iter>),
    RemoveGroupResponse(<RemoveGroupResponse as ToLeStream>::Iter),
}

impl Iterator for Iter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        #[expect(clippy::match_same_arms)]
        match self {
            Self::AddGroup(iter) => iter.next(),
            Self::ViewGroup(iter) => iter.next(),
            Self::GetGroupMembership(iter) => iter.next(),
            Self::RemoveGroup(iter) => iter.next(),
            Self::RemoveAllGroups(iter) => iter.next(),
            Self::AddGroupIfIdentifying(iter) => iter.next(),
            Self::AddGroupResponse(iter) => iter.next(),
            Self::ViewGroupResponse(iter) => iter.next(),
            Self::GetGroupMembershipResponse(iter) => iter.next(),
            Self::RemoveGroupResponse(iter) => iter.next(),
        }
    }
}
