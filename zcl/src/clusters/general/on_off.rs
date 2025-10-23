//! On/Off cluster.

pub use self::attribute::Attribute;
pub use self::commands::{DelayedAllOff, DyingLight, Effect, Off, OffWithEffect, On, Toggle};

mod attribute;
mod commands;

const CLUSTER_ID: u16 = 0x0006;
