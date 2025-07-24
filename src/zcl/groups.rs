//! The `Groups` cluster.

pub use attribute::{Attribute, NameSupport};
pub use commands::AddGroup;

mod attribute;
mod commands;

const CLUSTER_ID: u16 = 0x0004;
