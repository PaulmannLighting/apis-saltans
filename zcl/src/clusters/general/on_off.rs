//! On/Off cluster.

pub use attribute::Attribute;
pub use commands::{Off, OffWithEffect, On, Toggle};

mod attribute;
mod commands;

const CLUSTER_ID: u16 = 0x0006;
