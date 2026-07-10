//! Commands for the On/Off cluster.

use zb_core::Cluster;

pub use self::off::Off;
pub use self::off_with_effect::{DelayedAllOff, DyingLight, Effect, OffWithEffect};
pub use self::on::On;
pub use self::on_with_recall_global_scene::OnWithRecallGlobalScene;
pub use self::on_with_timed_off::{OnOffControl, OnWithTimedOff};
pub use self::toggle::Toggle;
use crate::macros::zcl_command_enum;

mod off;
mod off_with_effect;
mod on;
mod on_with_recall_global_scene;
mod on_with_timed_off;
mod toggle;

// Available On/Off cluster commands.
zcl_command_enum! {
    { Cluster::OnOff } => OnOff;
    Off(Off),
    On(On),
    Toggle(Toggle),
    OffWithEffect(OffWithEffect),
    OnWithRecallGlobalScene(OnWithRecallGlobalScene),
    OnWithTimedOff(OnWithTimedOff),
}
