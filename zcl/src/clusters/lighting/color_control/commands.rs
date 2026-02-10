use zigbee::Cluster;
use zigbee_macros::ParseZclFrame;

pub use self::color_loop_set::ColorLoopSet;
pub use self::enhanced_move_hue::EnhancedMoveHue;
pub use self::enhanced_move_to_hue::EnhancedMoveToHue;
pub use self::enhanced_move_to_hue_and_saturation::EnhancedMoveToHueAndSaturation;
pub use self::enhanced_step_hue::EnhancedStepHue;
pub use self::move_color::MoveColor;
pub use self::move_color_temperature::MoveColorTemperature;
pub use self::move_hue::MoveHue;
pub use self::move_saturation::MoveSaturation;
pub use self::move_to_color::MoveToColor;
pub use self::move_to_color_temperature::MoveToColorTemperature;
pub use self::move_to_hue::MoveToHue;
pub use self::move_to_hue_and_saturation::MoveToHueAndSaturation;
pub use self::move_to_saturation::MoveToSaturation;
pub use self::step_color::StepColor;
pub use self::step_color_temperature::StepColorTemperature;
pub use self::step_hue::StepHue;
pub use self::step_saturation::StepSaturation;
pub use self::stop_move_step::StopMoveStep;
use super::CLUSTER_ID;
use crate::CommandId;

pub mod color_loop_set;
mod enhanced_move_hue;
mod enhanced_move_to_hue;
mod enhanced_move_to_hue_and_saturation;
mod enhanced_step_hue;
mod move_color;
mod move_color_temperature;
pub mod move_hue;
pub mod move_saturation;
mod move_to_color;
mod move_to_color_temperature;
pub mod move_to_hue;
mod move_to_hue_and_saturation;
mod move_to_saturation;
mod step_color;
mod step_color_temperature;
pub mod step_hue;
pub mod step_saturation;
mod stop_move_step;

/// Enumeration of all commands in the `Color Control` cluster.
#[derive(Clone, Debug, Eq, PartialEq, Hash, ParseZclFrame)]
pub enum Command {
    /// Move to hue command.
    MoveToHue(MoveToHue),
    /// Move hue command.
    MoveHue(MoveHue),
    /// Step hue command.
    StepHue(StepHue),
    /// Move to saturation command.
    MoveToSaturation(MoveToSaturation),
    /// Move saturation command.
    MoveSaturation(MoveSaturation),
    /// Step saturation command.
    StepSaturation(StepSaturation),
    /// Move to hue and saturation command.
    MoveToHueAndSaturation(MoveToHueAndSaturation),
    /// Move to color command.
    MoveToColor(MoveToColor),
    /// Move color command.
    MoveColor(MoveColor),
    /// Step color command.
    StepColor(StepColor),
    /// Move to color temperature command.
    MoveToColorTemperature(MoveToColorTemperature),
    /// Enhanced move to hue command.
    EnhancedMoveToHue(EnhancedMoveToHue),
    /// Enhanced move hue command.
    EnhancedMoveHue(EnhancedMoveHue),
    /// Enhanced step hue command.
    EnhancedStepHue(EnhancedStepHue),
    /// Enhanced move to hue and saturation command.
    EnhancedMoveToHueAndSaturation(EnhancedMoveToHueAndSaturation),
    /// Color loop set command.
    ColorLoopSet(ColorLoopSet),
    /// Stop move step command.
    StopMoveStep(StopMoveStep),
    /// Move color temperature command.
    MoveColorTemperature(MoveColorTemperature),
    /// Step color temperature command.
    StepColorTemperature(StepColorTemperature),
}

impl Cluster for Command {
    const ID: u16 = CLUSTER_ID;
}

impl CommandId for Command {
    fn command_id(&self) -> u8 {
        match self {
            Self::MoveToHue(cmd) => cmd.command_id(),
            Self::MoveHue(cmd) => cmd.command_id(),
            Self::StepHue(cmd) => cmd.command_id(),
            Self::MoveToSaturation(cmd) => cmd.command_id(),
            Self::MoveSaturation(cmd) => cmd.command_id(),
            Self::StepSaturation(cmd) => cmd.command_id(),
            Self::MoveToHueAndSaturation(cmd) => cmd.command_id(),
            Self::MoveToColor(cmd) => cmd.command_id(),
            Self::MoveColor(cmd) => cmd.command_id(),
            Self::StepColor(cmd) => cmd.command_id(),
            Self::MoveToColorTemperature(cmd) => cmd.command_id(),
            Self::EnhancedMoveToHue(cmd) => cmd.command_id(),
            Self::EnhancedMoveHue(cmd) => cmd.command_id(),
            Self::EnhancedStepHue(cmd) => cmd.command_id(),
            Self::EnhancedMoveToHueAndSaturation(cmd) => cmd.command_id(),
            Self::ColorLoopSet(cmd) => cmd.command_id(),
            Self::StopMoveStep(cmd) => cmd.command_id(),
            Self::MoveColorTemperature(cmd) => cmd.command_id(),
            Self::StepColorTemperature(cmd) => cmd.command_id(),
        }
    }
}
