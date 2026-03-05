//! The `Groups` cluster.

pub use self::attribute::{NameSupport, readable};
pub use self::commands::{
    AddGroup, AddGroupIfIdentifying, AddGroupResponse, Command, GetGroupMembership,
    GetGroupMembershipResponse, RemoveAllGroups, RemoveGroup, RemoveGroupResponse, ViewGroup,
    ViewGroupResponse,
};

mod attribute;
mod commands;
mod types;

const CLUSTER_ID: u16 = 0x0004;
