//! On/Off cluster.

pub use self::attribute::Attribute;
pub use self::commands::{DelayedAllOff, DyingLight, Effect, Off, OffWithEffect, On, Toggle};

mod attribute;
mod commands;

const CLUSTER_ID: u16 = 0x0006;

/// Commands for the On/Off cluster.
#[derive(Debug)]
pub enum Command {
    /// Off command with no parameters.
    DelayedAllOff(DelayedAllOff),
    /// Off command with effect parameters.
    DyingLight(DyingLight),
    /// Off command with effect parameters.
    Effect(Effect),
    /// Off command.
    Off(Off),
    /// Off command with effect.
    OffWithEffect(OffWithEffect),
    /// On command.
    On(On),
    /// Toggle command.
    Toggle(Toggle),
}

/// Responses for the On/Off cluster.
#[derive(Debug)]
pub enum Response {}
