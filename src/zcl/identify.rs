//! Identify Cluster.

pub use attribute::Attribute;
pub use commands::{Identify, IdentifyQuery, TriggerEffect};

mod attribute;
mod commands;

const CLUSTER_ID: u16 = 0x0003;
