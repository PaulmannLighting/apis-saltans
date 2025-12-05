//! Scenes cluster.

pub use self::attribute::Attribute;
pub use self::scene_table::SceneTable;
pub use self::types::CurrentGroup;

mod attribute;
mod scene_table;
mod types;

const CLUSTER_ID: u16 = 0x0005;
