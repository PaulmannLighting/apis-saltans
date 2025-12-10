//! Commands for the On/Off cluster.

use le_stream::FromLeStream;
use zigbee::{DirectedCommand, Direction};

pub use self::off::Off;
pub use self::off_with_effect::{DelayedAllOff, DyingLight, Effect, OffWithEffect};
pub use self::on::On;
pub use self::toggle::Toggle;
use crate::ParseFrameError;

mod off;
mod off_with_effect;
mod on;
mod toggle;

/// Available On/Off cluster commands.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Command {
    /// On command.
    On(On),
    /// Off command.
    Off(Off),
    /// Off with Effect command.
    OffWithEffect(OffWithEffect),
    /// Toggle command.
    Toggle(Toggle),
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
            On::ID => On::from_le_stream(bytes)
                .map(Self::On)
                .ok_or(ParseFrameError::InsufficientPayload),
            Off::ID => Off::from_le_stream(bytes)
                .map(Self::Off)
                .ok_or(ParseFrameError::InsufficientPayload),
            OffWithEffect::ID => OffWithEffect::from_le_stream(bytes)
                .map(Self::OffWithEffect)
                .ok_or(ParseFrameError::InsufficientPayload),
            Toggle::ID => Toggle::from_le_stream(bytes)
                .map(Self::Toggle)
                .ok_or(ParseFrameError::InsufficientPayload),
            (command_id, _) => Err(ParseFrameError::InvalidCommandId(command_id)),
        }
    }
}
