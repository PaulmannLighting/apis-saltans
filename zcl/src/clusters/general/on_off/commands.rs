//! Commands for the On/Off cluster.

use zigbee::Cluster;
use zigbee_macros::ParseZclFrame;

pub use self::off::Off;
pub use self::off_with_effect::{DelayedAllOff, DyingLight, Effect, OffWithEffect};
pub use self::on::On;
pub use self::on_with_recall_global_scene::OnWithRecallGlobalScene;
pub use self::on_with_timed_off::{OnOffControl, OnWithTimedOff};
pub use self::toggle::Toggle;
use crate::CommandId;

mod off;
mod off_with_effect;
mod on;
mod on_with_recall_global_scene;
mod on_with_timed_off;
mod toggle;

/// Available On/Off cluster commands.
#[expect(variant_size_differences)]
#[derive(Clone, Debug, Eq, PartialEq, Hash, ParseZclFrame)]
pub enum Command {
    /// Off command.
    Off(Off),
    /// On command.
    On(On),
    /// Toggle command.
    Toggle(Toggle),
    /// Off with Effect command.
    OffWithEffect(OffWithEffect),
    /// On with Recall Global Scene command.
    OnWithRecallGlobalScene(OnWithRecallGlobalScene),
    /// On with Timed Off command.
    OnWithTimedOff(OnWithTimedOff),
}

impl Cluster for Command {
    const ID: u16 = super::CLUSTER_ID;
}

impl CommandId for Command {
    fn command_id(&self) -> u8 {
        match self {
            Self::Toggle(cmd) => cmd.command_id(),
            Self::Off(cmd) => cmd.command_id(),
            Self::On(cmd) => cmd.command_id(),
            Self::OffWithEffect(cmd) => cmd.command_id(),
            Self::OnWithRecallGlobalScene(cmd) => cmd.command_id(),
            Self::OnWithTimedOff(cmd) => cmd.command_id(),
        }
    }
}
