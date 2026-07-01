use le_stream::ToLeStream;
use apis_saltans_core::{ClusterId, ClusterSpecific, Direction};
use apis_saltans_macros::ParseZclFrame;

pub use self::status_change::StatusChange;
use crate::{CommandDispatch, Scope};

mod status_change;

/// IAS Zone cluster commands.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, ParseZclFrame)]
pub enum Command {
    /// Zone status change command.
    StatusChange(StatusChange),
}

impl ClusterSpecific for Command {
    const CLUSTER: ClusterId = ClusterId::IasZone;
}

impl CommandDispatch for Command {
    fn command_id(&self) -> u8 {
        match self {
            Self::StatusChange(cmd) => cmd.command_id(),
        }
    }

    fn scope(&self) -> Scope {
        match self {
            Self::StatusChange(cmd) => cmd.scope(),
        }
    }

    fn direction(&self) -> Direction {
        match self {
            Self::StatusChange(cmd) => cmd.direction(),
        }
    }

    fn disable_default_response(&self) -> bool {
        match self {
            Self::StatusChange(cmd) => cmd.disable_default_response(),
        }
    }
}

impl ToLeStream for Command {
    type Iter = Iter;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::StatusChange(cmd) => Iter::StatusChange(cmd.to_le_stream()),
        }
    }
}

impl From<StatusChange> for Command {
    fn from(value: StatusChange) -> Self {
        Self::StatusChange(value)
    }
}

#[derive(Debug)]
pub enum Iter {
    StatusChange(<StatusChange as ToLeStream>::Iter),
}

impl Iterator for Iter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::StatusChange(iter) => iter.next(),
        }
    }
}
