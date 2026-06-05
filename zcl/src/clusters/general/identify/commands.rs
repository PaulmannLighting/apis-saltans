use le_stream::ToLeStream;
use zigbee::{ClusterId, ClusterSpecific, Direction};
use zigbee_macros::ParseZclFrame;

pub use self::identify::Identify;
pub use self::identify_query::IdentifyQuery;
pub use self::identify_query_response::IdentifyQueryResponse;
pub use self::trigger_effect::{EffectIdentifier, EffectVariant, TriggerEffect};
use crate::{CommandDispatch, Scope};

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

impl ClusterSpecific for Command {
    const CLUSTER: ClusterId = ClusterId::Identify;
}

impl From<Command> for crate::Cluster {
    fn from(command: Command) -> Self {
        Self::Identify(command)
    }
}

impl From<Identify> for Command {
    fn from(command: Identify) -> Self {
        Self::Identify(command)
    }
}

impl From<IdentifyQuery> for Command {
    fn from(command: IdentifyQuery) -> Self {
        Self::IdentifyQuery(command)
    }
}

impl From<TriggerEffect> for Command {
    fn from(command: TriggerEffect) -> Self {
        Self::TriggerEffect(command)
    }
}

impl From<IdentifyQueryResponse> for Command {
    fn from(response: IdentifyQueryResponse) -> Self {
        Self::IdentifyQueryResponse(response)
    }
}

impl CommandDispatch for Command {
    fn command_id(&self) -> u8 {
        match self {
            Self::Identify(cmd) => cmd.command_id(),
            Self::IdentifyQuery(cmd) => cmd.command_id(),
            Self::TriggerEffect(cmd) => cmd.command_id(),
            Self::IdentifyQueryResponse(cmd) => cmd.command_id(),
        }
    }

    fn scope(&self) -> Scope {
        match self {
            Self::Identify(cmd) => cmd.scope(),
            Self::IdentifyQuery(cmd) => cmd.scope(),
            Self::TriggerEffect(cmd) => cmd.scope(),
            Self::IdentifyQueryResponse(cmd) => cmd.scope(),
        }
    }

    fn direction(&self) -> Direction {
        match self {
            Self::Identify(cmd) => cmd.direction(),
            Self::IdentifyQuery(cmd) => cmd.direction(),
            Self::TriggerEffect(cmd) => cmd.direction(),
            Self::IdentifyQueryResponse(cmd) => cmd.direction(),
        }
    }

    fn disable_default_response(&self) -> bool {
        match self {
            Self::Identify(cmd) => cmd.disable_default_response(),
            Self::IdentifyQuery(cmd) => cmd.disable_default_response(),
            Self::TriggerEffect(cmd) => cmd.disable_default_response(),
            Self::IdentifyQueryResponse(cmd) => cmd.disable_default_response(),
        }
    }
}

impl ToLeStream for Command {
    type Iter = Iter;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::Identify(cmd) => Iter::Identify(cmd.to_le_stream()),
            Self::IdentifyQuery(cmd) => Iter::IdentifyQuery(cmd.to_le_stream()),
            Self::TriggerEffect(cmd) => Iter::TriggerEffect(cmd.to_le_stream()),
            Self::IdentifyQueryResponse(cmd) => Iter::IdentifyQueryResponse(cmd.to_le_stream()),
        }
    }
}

#[derive(Debug)]
pub enum Iter {
    Identify(<Identify as ToLeStream>::Iter),
    IdentifyQuery(<IdentifyQuery as ToLeStream>::Iter),
    TriggerEffect(<TriggerEffect as ToLeStream>::Iter),
    IdentifyQueryResponse(<IdentifyQueryResponse as ToLeStream>::Iter),
}

impl Iterator for Iter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        #[expect(clippy::match_same_arms)]
        match self {
            Self::Identify(iter) => iter.next(),
            Self::IdentifyQuery(iter) => iter.next(),
            Self::TriggerEffect(iter) => iter.next(),
            Self::IdentifyQueryResponse(iter) => iter.next(),
        }
    }
}
