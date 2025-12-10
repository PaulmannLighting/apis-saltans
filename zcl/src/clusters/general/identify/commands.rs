use zigbee::Cluster;
use zigbee_macros::ParseZclFrame;

pub use self::identify::Identify;
pub use self::identify_query::IdentifyQuery;
pub use self::identify_query_response::IdentifyQueryResponse;
pub use self::trigger_effect::{EffectIdentifier, EffectVariant, TriggerEffect};

mod identify;
mod identify_query;
mod identify_query_response;
mod trigger_effect;

/// Available Identify cluster commands.
#[derive(Clone, Debug, Eq, PartialEq, Hash, ParseZclFrame)]
pub enum Command {
    /// Identify command.
    Identify(Identify),
    /// Identify Query command.
    IdentifyQuery(IdentifyQuery),
    /// Trigger Effect command.
    TriggerEffect(TriggerEffect),
    /// Identify Query Response command.
    IdentifyQueryResponse(IdentifyQueryResponse),
}

impl Cluster for Command {
    const ID: u16 = super::CLUSTER_ID;
}
