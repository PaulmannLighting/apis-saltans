//! The `Groups` cluster.

pub use self::attribute::{Attribute, NameSupport};
pub use self::commands::{
    AddGroup, AddGroupIfIdentifying, AddGroupResponse, GetGroupMembership,
    GetGroupMembershipResponse, RemoveAllGroups, RemoveGroup, RemoveGroupResponse, ViewGroup,
    ViewGroupResponse,
};

mod attribute;
mod commands;
mod types;

const CLUSTER_ID: u16 = 0x0004;

/// Groups Cluster commands.
#[expect(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Command {
    /// Add Group command.
    AddGroup(AddGroup),
    /// Add Group If Identifying command.
    AddGroupIfIdentifying(AddGroupIfIdentifying),
    /// Get Group Membership command.
    GetGroupMembership(GetGroupMembership),
    /// Remove All Groups command.
    RemoveAllGroups(RemoveAllGroups),
    /// Remove Group command.
    RemoveGroup(RemoveGroup),
    /// View Group command.
    ViewGroup(ViewGroup),
}

/// Groups Cluster responses.
#[expect(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Response {
    /// Add Group response.
    AddGroup(AddGroupResponse),
    /// Get Group Membership response.
    GetGroupMembership(GetGroupMembershipResponse),
    /// Remove Group response.
    RemoveGroup(RemoveGroupResponse),
    /// View Group response.
    ViewGroup(ViewGroupResponse),
}
