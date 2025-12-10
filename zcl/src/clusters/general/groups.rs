//! The `Groups` cluster.

pub use self::attribute::{Attribute, NameSupport};
pub use self::commands::{
    AddGroup, AddGroupIfIdentifying, AddGroupResponse, Command, GetGroupMembership,
    GetGroupMembershipResponse, RemoveAllGroups, RemoveGroup, RemoveGroupResponse, ViewGroup,
    ViewGroupResponse,
};

mod attribute;
mod commands;
mod types;

/// Cluster ID for the Groups cluster.
pub const CLUSTER_ID: u16 = 0x0004;
