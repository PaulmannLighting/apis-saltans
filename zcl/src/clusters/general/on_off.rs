//! On/Off cluster.

pub use self::attribute::Attribute;
pub use self::commands::{
    Command, DelayedAllOff, DyingLight, Effect, Off, OffWithEffect, On, OnOffControl,
    OnWithRecallGlobalScene, OnWithTimedOff, Toggle,
};

mod attribute;
mod commands;

const CLUSTER_ID: u16 = 0x0006;
