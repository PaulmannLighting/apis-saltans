use zigbee::Cluster;
use zigbee_macros::ParseZclFrame;

pub use self::r#move::Move;
pub use self::move_to_closest_frequency::MoveToClosestFrequency;
pub use self::move_to_level::MoveToLevel;
pub use self::move_to_level_with_on_off::MoveToLevelWithOnOff;
pub use self::move_with_on_off::MoveWithOnOff;
pub use self::step::Step;
pub use self::step_with_on_off::StepWithOnOff;
pub use self::stop::Stop;
pub use self::stop_with_on_off::StopWithOnOff;
use super::CLUSTER_ID;
use crate::CommandId;

mod r#move;
mod move_to_closest_frequency;
mod move_to_level;
mod move_to_level_with_on_off;
mod move_with_on_off;
mod step;
mod step_with_on_off;
mod stop;
mod stop_with_on_off;

/// Available On/Off cluster commands.
#[derive(Clone, Debug, Eq, PartialEq, Hash, ParseZclFrame)]
pub enum Command {
    /// Move to level command.
    MoveToLevel(MoveToLevel),
    /// Move command.
    Move(Move),
    /// Step command.
    Step(Step),
    /// Stop command.
    Stop(Stop),
    /// Move to level with on/off command.
    MoveToLevelWithOnOff(MoveToLevelWithOnOff),
    /// Move with on/off command.
    MoveWithOnOff(MoveWithOnOff),
    /// Step with on/off command.
    StepWithOnOff(StepWithOnOff),
    /// Stop with on/off command.
    StopWithOnOff(StopWithOnOff),
    /// Move to the closest frequency command.
    MoveToClosestFrequency(MoveToClosestFrequency),
}

impl Cluster for Command {
    const ID: u16 = CLUSTER_ID;
}

impl CommandId for Command {
    fn command_id(&self) -> u8 {
        match self {
            Self::MoveToLevel(cmd) => cmd.command_id(),
            Self::Move(cmd) => cmd.command_id(),
            Self::Step(cmd) => cmd.command_id(),
            Self::Stop(cmd) => cmd.command_id(),
            Self::MoveToLevelWithOnOff(cmd) => cmd.command_id(),
            Self::MoveWithOnOff(cmd) => cmd.command_id(),
            Self::StepWithOnOff(cmd) => cmd.command_id(),
            Self::StopWithOnOff(cmd) => cmd.command_id(),
            Self::MoveToClosestFrequency(cmd) => cmd.command_id(),
        }
    }
}
