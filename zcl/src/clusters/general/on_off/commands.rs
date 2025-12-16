//! Commands for the On/Off cluster.

use zigbee::Cluster;
use zigbee_macros::ParseZclFrame;

pub use self::off::Off;
pub use self::off_with_effect::{DelayedAllOff, DyingLight, Effect, OffWithEffect};
pub use self::on::On;
pub use self::toggle::Toggle;
use crate::CommandId;

mod off;
mod off_with_effect;
mod on;
mod toggle;

/// Available On/Off cluster commands.
#[derive(Clone, Debug, Eq, PartialEq, Hash, ParseZclFrame)]
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

impl Cluster for Command {
    const ID: u16 = super::CLUSTER_ID;
}

impl CommandId for Command {
    fn command_id(&self) -> u8 {
        match self {
            Self::On(cmd) => cmd.command_id(),
            Self::Off(cmd) => cmd.command_id(),
            Self::OffWithEffect(cmd) => cmd.command_id(),
            Self::Toggle(cmd) => cmd.command_id(),
        }
    }
}
