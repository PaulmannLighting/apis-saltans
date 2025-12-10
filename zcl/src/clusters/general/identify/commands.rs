use le_stream::FromLeStream;
use zigbee::{DirectedCommand, Direction};

pub use self::identify::Identify;
pub use self::identify_query::IdentifyQuery;
pub use self::identify_query_response::IdentifyQueryResponse;
pub use self::trigger_effect::{EffectIdentifier, EffectVariant, TriggerEffect};
use crate::ParseFrameError;

mod identify;
mod identify_query;
mod identify_query_response;
mod trigger_effect;

/// Available Identify cluster commands.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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

impl Command {
    pub fn from_le_stream<T>(
        command_id: u8,
        direction: Direction,
        bytes: T,
    ) -> Result<Self, ParseFrameError>
    where
        T: Iterator<Item = u8>,
    {
        match (command_id, direction) {
            Identify::ID => Identify::from_le_stream(bytes)
                .map(Command::Identify)
                .ok_or(ParseFrameError::InsufficientPayload),
            IdentifyQuery::ID => IdentifyQuery::from_le_stream(bytes)
                .map(Command::IdentifyQuery)
                .ok_or(ParseFrameError::InsufficientPayload),
            TriggerEffect::ID => TriggerEffect::from_le_stream(bytes)
                .map(Command::TriggerEffect)
                .ok_or(ParseFrameError::InsufficientPayload),
            IdentifyQueryResponse::ID => IdentifyQueryResponse::from_le_stream(bytes)
                .map(Command::IdentifyQueryResponse)
                .ok_or(ParseFrameError::InsufficientPayload),
            (command_id, _) => Err(ParseFrameError::InvalidCommandId(command_id)),
        }
    }
}
