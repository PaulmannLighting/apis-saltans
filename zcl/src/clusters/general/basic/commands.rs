//! Commands for the Basic cluster.

use le_stream::ToLeStream;
use zigbee::{Cluster, Direction};
use zigbee_macros::ParseZclFrame;

pub use self::reset_to_factory_defaults::ResetToFactoryDefaults;
use crate::{CommandDispatch, Scope};

mod reset_to_factory_defaults;

/// Available commands for the Basic cluster.
#[derive(Clone, Debug, Eq, PartialEq, Hash, ParseZclFrame)]
pub enum Command {
    /// Reset to Factory Defaults command.
    ResetToFactoryDefaults(ResetToFactoryDefaults),
}

impl Cluster for Command {
    const ID: u16 = super::CLUSTER_ID;
}

impl From<Command> for crate::Cluster {
    fn from(command: Command) -> Self {
        Self::Basic(command)
    }
}

impl From<ResetToFactoryDefaults> for Command {
    fn from(command: ResetToFactoryDefaults) -> Self {
        Self::ResetToFactoryDefaults(command)
    }
}

impl CommandDispatch for Command {
    fn command_id(&self) -> u8 {
        match self {
            Self::ResetToFactoryDefaults(cmd) => cmd.command_id(),
        }
    }

    fn scope(&self) -> Scope {
        match self {
            Self::ResetToFactoryDefaults(cmd) => cmd.scope(),
        }
    }

    fn direction(&self) -> Direction {
        match self {
            Self::ResetToFactoryDefaults(cmd) => cmd.direction(),
        }
    }

    fn disable_default_response(&self) -> bool {
        match self {
            Self::ResetToFactoryDefaults(cmd) => cmd.disable_default_response(),
        }
    }
}

impl ToLeStream for Command {
    type Iter = Iter;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::ResetToFactoryDefaults(cmd) => Iter::ResetToFactoryDefaults(cmd.to_le_stream()),
        }
    }
}

#[expect(missing_docs)]
#[derive(Debug)]
pub enum Iter {
    ResetToFactoryDefaults(<ResetToFactoryDefaults as ToLeStream>::Iter),
}

impl Iterator for Iter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::ResetToFactoryDefaults(iter) => iter.next(),
        }
    }
}
