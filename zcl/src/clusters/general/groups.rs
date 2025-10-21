//! The `Groups` cluster.

pub use attribute::{Attribute, NameSupport};
pub use commands::{
    AddGroup, AddGroupIfIdentifying, AddGroupResponse, GetGroupMembership,
    GetGroupMembershipResponse, RemoveAllGroups, RemoveGroup, RemoveGroupResponse, ViewGroup,
    ViewGroupResponse,
};

mod attribute;
mod commands;
mod types;

const CLUSTER_ID: u16 = 0x0004;
