use apis_saltans_core::ClusterId;

pub use self::identify::Identify;
pub use self::identify_query::IdentifyQuery;
pub use self::identify_query_response::IdentifyQueryResponse;
pub use self::trigger_effect::{EffectIdentifier, EffectVariant, TriggerEffect};
use crate::macros::zcl_command_enum;

mod identify;
mod identify_query;
mod identify_query_response;
mod trigger_effect;

// Available Identify cluster commands.
zcl_command_enum! {
    { ClusterId::Identify } => Identify;
    Identify(Identify),
    IdentifyQuery(IdentifyQuery),
    TriggerEffect(TriggerEffect),
    IdentifyQueryResponse(IdentifyQueryResponse),
}
