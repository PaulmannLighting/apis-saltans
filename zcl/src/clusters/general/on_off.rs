//! On/Off cluster.

pub use self::attribute::Attribute;
pub use self::commands::{
    Command, DelayedAllOff, DyingLight, Effect, Off, OffWithEffect, On, Toggle,
};

mod attribute;
mod commands;

/// On/Off cluster ID.
pub const CLUSTER_ID: u16 = 0x0006;
