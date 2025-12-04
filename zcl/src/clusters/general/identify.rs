//! Identify Cluster.

pub use self::attribute::Attribute;
pub use self::commands::{
    EffectIdentifier, EffectVariant, Identify, IdentifyQuery, IdentifyQueryResponse, TriggerEffect,
};

mod attribute;
mod commands;

const CLUSTER_ID: u16 = 0x0003;

/// Identify Cluster commands.
#[derive(Debug)]
pub enum Command {
    /// Identify command.
    Identify(Identify),
    /// Identify Query command.
    IdentifyQuery(IdentifyQuery),
    /// Trigger Effect command.
    TriggerEffect(TriggerEffect),
}

/// Identify Cluster responses.
#[derive(Debug)]
pub enum Response {
    /// Identify Query response.
    IdentifyQuery(IdentifyQueryResponse),
}
