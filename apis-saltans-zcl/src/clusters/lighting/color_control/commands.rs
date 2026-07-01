use le_stream::ToLeStream;
use apis_saltans_core::{ClusterId, ClusterSpecific, Direction};
use apis_saltans_macros::ParseZclFrame;

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
use crate::{CommandDispatch, Scope};

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

impl ClusterSpecific for Command {
    const CLUSTER: ClusterId = ClusterId::ColorControl;
}

impl From<Command> for crate::Cluster {
    fn from(command: Command) -> Self {
        Self::ColorControl(command)
    }
}

impl From<MoveToHue> for Command {
    fn from(command: MoveToHue) -> Self {
        Self::MoveToHue(command)
    }
}

impl From<MoveHue> for Command {
    fn from(command: MoveHue) -> Self {
        Self::MoveHue(command)
    }
}

impl From<StepHue> for Command {
    fn from(command: StepHue) -> Self {
        Self::StepHue(command)
    }
}

impl From<MoveToSaturation> for Command {
    fn from(command: MoveToSaturation) -> Self {
        Self::MoveToSaturation(command)
    }
}

impl From<MoveSaturation> for Command {
    fn from(command: MoveSaturation) -> Self {
        Self::MoveSaturation(command)
    }
}

impl From<StepSaturation> for Command {
    fn from(command: StepSaturation) -> Self {
        Self::StepSaturation(command)
    }
}

impl From<MoveToHueAndSaturation> for Command {
    fn from(command: MoveToHueAndSaturation) -> Self {
        Self::MoveToHueAndSaturation(command)
    }
}

impl From<MoveToColor> for Command {
    fn from(command: MoveToColor) -> Self {
        Self::MoveToColor(command)
    }
}

impl From<MoveColor> for Command {
    fn from(command: MoveColor) -> Self {
        Self::MoveColor(command)
    }
}

impl From<StepColor> for Command {
    fn from(command: StepColor) -> Self {
        Self::StepColor(command)
    }
}

impl From<MoveToColorTemperature> for Command {
    fn from(command: MoveToColorTemperature) -> Self {
        Self::MoveToColorTemperature(command)
    }
}

impl From<EnhancedMoveToHue> for Command {
    fn from(command: EnhancedMoveToHue) -> Self {
        Self::EnhancedMoveToHue(command)
    }
}

impl From<EnhancedMoveHue> for Command {
    fn from(command: EnhancedMoveHue) -> Self {
        Self::EnhancedMoveHue(command)
    }
}

impl From<EnhancedStepHue> for Command {
    fn from(command: EnhancedStepHue) -> Self {
        Self::EnhancedStepHue(command)
    }
}

impl From<EnhancedMoveToHueAndSaturation> for Command {
    fn from(command: EnhancedMoveToHueAndSaturation) -> Self {
        Self::EnhancedMoveToHueAndSaturation(command)
    }
}

impl From<ColorLoopSet> for Command {
    fn from(command: ColorLoopSet) -> Self {
        Self::ColorLoopSet(command)
    }
}

impl From<StopMoveStep> for Command {
    fn from(command: StopMoveStep) -> Self {
        Self::StopMoveStep(command)
    }
}

impl From<MoveColorTemperature> for Command {
    fn from(command: MoveColorTemperature) -> Self {
        Self::MoveColorTemperature(command)
    }
}

impl From<StepColorTemperature> for Command {
    fn from(command: StepColorTemperature) -> Self {
        Self::StepColorTemperature(command)
    }
}

impl CommandDispatch for Command {
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

    fn scope(&self) -> Scope {
        match self {
            Self::MoveToHue(cmd) => cmd.scope(),
            Self::MoveHue(cmd) => cmd.scope(),
            Self::StepHue(cmd) => cmd.scope(),
            Self::MoveToSaturation(cmd) => cmd.scope(),
            Self::MoveSaturation(cmd) => cmd.scope(),
            Self::StepSaturation(cmd) => cmd.scope(),
            Self::MoveToHueAndSaturation(cmd) => cmd.scope(),
            Self::MoveToColor(cmd) => cmd.scope(),
            Self::MoveColor(cmd) => cmd.scope(),
            Self::StepColor(cmd) => cmd.scope(),
            Self::MoveToColorTemperature(cmd) => cmd.scope(),
            Self::EnhancedMoveToHue(cmd) => cmd.scope(),
            Self::EnhancedMoveHue(cmd) => cmd.scope(),
            Self::EnhancedStepHue(cmd) => cmd.scope(),
            Self::EnhancedMoveToHueAndSaturation(cmd) => cmd.scope(),
            Self::ColorLoopSet(cmd) => cmd.scope(),
            Self::StopMoveStep(cmd) => cmd.scope(),
            Self::MoveColorTemperature(cmd) => cmd.scope(),
            Self::StepColorTemperature(cmd) => cmd.scope(),
        }
    }

    fn direction(&self) -> Direction {
        match self {
            Self::MoveToHue(cmd) => CommandDispatch::direction(cmd),
            Self::MoveHue(cmd) => CommandDispatch::direction(cmd),
            Self::StepHue(cmd) => CommandDispatch::direction(cmd),
            Self::MoveToSaturation(cmd) => CommandDispatch::direction(cmd),
            Self::MoveSaturation(cmd) => CommandDispatch::direction(cmd),
            Self::StepSaturation(cmd) => CommandDispatch::direction(cmd),
            Self::MoveToHueAndSaturation(cmd) => CommandDispatch::direction(cmd),
            Self::MoveToColor(cmd) => CommandDispatch::direction(cmd),
            Self::MoveColor(cmd) => CommandDispatch::direction(cmd),
            Self::StepColor(cmd) => CommandDispatch::direction(cmd),
            Self::MoveToColorTemperature(cmd) => CommandDispatch::direction(cmd),
            Self::EnhancedMoveToHue(cmd) => CommandDispatch::direction(cmd),
            Self::EnhancedMoveHue(cmd) => CommandDispatch::direction(cmd),
            Self::EnhancedStepHue(cmd) => CommandDispatch::direction(cmd),
            Self::EnhancedMoveToHueAndSaturation(cmd) => CommandDispatch::direction(cmd),
            Self::ColorLoopSet(cmd) => CommandDispatch::direction(cmd),
            Self::StopMoveStep(cmd) => CommandDispatch::direction(cmd),
            Self::MoveColorTemperature(cmd) => CommandDispatch::direction(cmd),
            Self::StepColorTemperature(cmd) => CommandDispatch::direction(cmd),
        }
    }

    fn disable_default_response(&self) -> bool {
        match self {
            Self::MoveToHue(cmd) => cmd.disable_default_response(),
            Self::MoveHue(cmd) => cmd.disable_default_response(),
            Self::StepHue(cmd) => cmd.disable_default_response(),
            Self::MoveToSaturation(cmd) => cmd.disable_default_response(),
            Self::MoveSaturation(cmd) => cmd.disable_default_response(),
            Self::StepSaturation(cmd) => cmd.disable_default_response(),
            Self::MoveToHueAndSaturation(cmd) => cmd.disable_default_response(),
            Self::MoveToColor(cmd) => cmd.disable_default_response(),
            Self::MoveColor(cmd) => cmd.disable_default_response(),
            Self::StepColor(cmd) => cmd.disable_default_response(),
            Self::MoveToColorTemperature(cmd) => cmd.disable_default_response(),
            Self::EnhancedMoveToHue(cmd) => cmd.disable_default_response(),
            Self::EnhancedMoveHue(cmd) => cmd.disable_default_response(),
            Self::EnhancedStepHue(cmd) => cmd.disable_default_response(),
            Self::EnhancedMoveToHueAndSaturation(cmd) => cmd.disable_default_response(),
            Self::ColorLoopSet(cmd) => cmd.disable_default_response(),
            Self::StopMoveStep(cmd) => cmd.disable_default_response(),
            Self::MoveColorTemperature(cmd) => cmd.disable_default_response(),
            Self::StepColorTemperature(cmd) => cmd.disable_default_response(),
        }
    }
}

impl ToLeStream for Command {
    type Iter = Iter;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::MoveToHue(cmd) => Iter::MoveToHue(cmd.to_le_stream()),
            Self::MoveHue(cmd) => Iter::MoveHue(cmd.to_le_stream()),
            Self::StepHue(cmd) => Iter::StepHue(cmd.to_le_stream()),
            Self::MoveToSaturation(cmd) => Iter::MoveToSaturation(cmd.to_le_stream()),
            Self::MoveSaturation(cmd) => Iter::MoveSaturation(cmd.to_le_stream()),
            Self::StepSaturation(cmd) => Iter::StepSaturation(cmd.to_le_stream()),
            Self::MoveToHueAndSaturation(cmd) => Iter::MoveToHueAndSaturation(cmd.to_le_stream()),
            Self::MoveToColor(cmd) => Iter::MoveToColor(cmd.to_le_stream()),
            Self::MoveColor(cmd) => Iter::MoveColor(cmd.to_le_stream()),
            Self::StepColor(cmd) => Iter::StepColor(cmd.to_le_stream()),
            Self::MoveToColorTemperature(cmd) => Iter::MoveToColorTemperature(cmd.to_le_stream()),
            Self::EnhancedMoveToHue(cmd) => Iter::EnhancedMoveToHue(cmd.to_le_stream()),
            Self::EnhancedMoveHue(cmd) => Iter::EnhancedMoveHue(cmd.to_le_stream()),
            Self::EnhancedStepHue(cmd) => Iter::EnhancedStepHue(cmd.to_le_stream()),
            Self::EnhancedMoveToHueAndSaturation(cmd) => {
                Iter::EnhancedMoveToHueAndSaturation(cmd.to_le_stream())
            }
            Self::ColorLoopSet(cmd) => Iter::ColorLoopSet(cmd.to_le_stream()),
            Self::StopMoveStep(cmd) => Iter::StopMoveStep(cmd.to_le_stream()),
            Self::MoveColorTemperature(cmd) => Iter::MoveColorTemperature(cmd.to_le_stream()),
            Self::StepColorTemperature(cmd) => Iter::StepColorTemperature(cmd.to_le_stream()),
        }
    }
}

#[derive(Debug)]
pub enum Iter {
    MoveToHue(<MoveToHue as ToLeStream>::Iter),
    MoveHue(<MoveHue as ToLeStream>::Iter),
    StepHue(<StepHue as ToLeStream>::Iter),
    MoveToSaturation(<MoveToSaturation as ToLeStream>::Iter),
    MoveSaturation(<MoveSaturation as ToLeStream>::Iter),
    StepSaturation(<StepSaturation as ToLeStream>::Iter),
    MoveToHueAndSaturation(<MoveToHueAndSaturation as ToLeStream>::Iter),
    MoveToColor(<MoveToColor as ToLeStream>::Iter),
    MoveColor(<MoveColor as ToLeStream>::Iter),
    StepColor(<StepColor as ToLeStream>::Iter),
    MoveToColorTemperature(<MoveToColorTemperature as ToLeStream>::Iter),
    EnhancedMoveToHue(<EnhancedMoveToHue as ToLeStream>::Iter),
    EnhancedMoveHue(<EnhancedMoveHue as ToLeStream>::Iter),
    EnhancedStepHue(<EnhancedStepHue as ToLeStream>::Iter),
    EnhancedMoveToHueAndSaturation(<EnhancedMoveToHueAndSaturation as ToLeStream>::Iter),
    ColorLoopSet(<ColorLoopSet as ToLeStream>::Iter),
    StopMoveStep(<StopMoveStep as ToLeStream>::Iter),
    MoveColorTemperature(<MoveColorTemperature as ToLeStream>::Iter),
    StepColorTemperature(<StepColorTemperature as ToLeStream>::Iter),
}

impl Iterator for Iter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        #[expect(clippy::match_same_arms)]
        match self {
            Self::MoveToHue(iter) => iter.next(),
            Self::MoveHue(iter) => iter.next(),
            Self::StepHue(iter) => iter.next(),
            Self::MoveToSaturation(iter) => iter.next(),
            Self::MoveSaturation(iter) => iter.next(),
            Self::StepSaturation(iter) => iter.next(),
            Self::MoveToHueAndSaturation(iter) => iter.next(),
            Self::MoveToColor(iter) => iter.next(),
            Self::MoveColor(iter) => iter.next(),
            Self::StepColor(iter) => iter.next(),
            Self::MoveToColorTemperature(iter) => iter.next(),
            Self::EnhancedMoveToHue(iter) => iter.next(),
            Self::EnhancedMoveHue(iter) => iter.next(),
            Self::EnhancedStepHue(iter) => iter.next(),
            Self::EnhancedMoveToHueAndSaturation(iter) => iter.next(),
            Self::ColorLoopSet(iter) => iter.next(),
            Self::StopMoveStep(iter) => iter.next(),
            Self::MoveColorTemperature(iter) => iter.next(),
            Self::StepColorTemperature(iter) => iter.next(),
        }
    }
}
