//! Scenes cluster.

pub use self::attribute::Attribute;
pub use self::scene_table::SceneTable;

mod attribute;
mod scene_table;

const CLUSTER_ID: u16 = 0x0005;
