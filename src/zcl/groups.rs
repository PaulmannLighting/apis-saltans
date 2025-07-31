//! The `Groups` cluster.

pub use attribute::{Attribute, NameSupport};
pub use commands::{
    AddGroup, AddGroupIfIdentifying, AddGroupResponse, GetGroupMembership, RemoveAllGroups,
    RemoveGroup, ViewGroup, ViewGroupResponse,
};

mod attribute;
mod commands;

const CLUSTER_ID: u16 = 0x0004;
