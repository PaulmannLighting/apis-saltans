//! The `Groups` cluster.

pub use attribute::{Attribute, NameSupport};
pub use commands::{
    AddGroup, AddGroupIfIdentifying, GetGroupMembership, RemoveAllGroups, RemoveGroup, ViewGroup,
};

mod attribute;
mod commands;

const CLUSTER_ID: u16 = 0x0004;
