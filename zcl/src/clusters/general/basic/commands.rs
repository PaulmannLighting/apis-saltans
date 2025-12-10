//! Commands for the Basic cluster.

use le_stream::FromLeStream;
use zigbee::{DirectedCommand, Direction};

pub use self::reset_to_factory_defaults::ResetToFactoryDefaults;
use crate::ParseFrameError;

mod reset_to_factory_defaults;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Command {
    /// Reset to Factory Defaults command.
    ResetToFactoryDefaults(ResetToFactoryDefaults),
}

impl Command {
    pub fn from_le_stream<T>(
        command_id: u8,
        direction: Direction,
        mut bytes: T,
    ) -> Result<Self, ParseFrameError>
    where
        T: Iterator<Item = u8>,
    {
        match (command_id, direction) {
            ResetToFactoryDefaults::ID => ResetToFactoryDefaults::from_le_stream(&mut bytes)
                .map(Self::ResetToFactoryDefaults)
                .ok_or(ParseFrameError::InsufficientPayload),
            (command_id, _) => Err(ParseFrameError::InvalidCommandId(command_id)),
        }
    }
}
