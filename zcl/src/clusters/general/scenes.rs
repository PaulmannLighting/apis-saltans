//! Scenes cluster.

pub use self::attribute::Attribute;
pub use self::scene_table::SceneTable;
pub use self::types::CurrentGroup;

mod attribute;
mod scene_table;
mod types;

const CLUSTER_ID: u16 = 0x0005;

/// Commands for the Scenes cluster.
#[derive(Debug)]
pub enum Command {}

/// Responses for the Scenes cluster.
#[derive(Debug)]
pub enum Response {}
