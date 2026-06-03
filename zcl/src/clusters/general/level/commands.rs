use le_stream::ToLeStream;
use zigbee::{Cluster, Direction};
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
use crate::{CommandDispatch, Scope};

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

impl CommandDispatch for Command {
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

    fn scope(&self) -> Scope {
        match self {
            Self::MoveToLevel(cmd) => cmd.scope(),
            Self::Move(cmd) => cmd.scope(),
            Self::Step(cmd) => cmd.scope(),
            Self::Stop(cmd) => cmd.scope(),
            Self::MoveToLevelWithOnOff(cmd) => cmd.scope(),
            Self::MoveWithOnOff(cmd) => cmd.scope(),
            Self::StepWithOnOff(cmd) => cmd.scope(),
            Self::StopWithOnOff(cmd) => cmd.scope(),
            Self::MoveToClosestFrequency(cmd) => cmd.scope(),
        }
    }

    fn direction(&self) -> Direction {
        match self {
            Self::MoveToLevel(cmd) => cmd.direction(),
            Self::Move(cmd) => cmd.direction(),
            Self::Step(cmd) => cmd.direction(),
            Self::Stop(cmd) => cmd.direction(),
            Self::MoveToLevelWithOnOff(cmd) => cmd.direction(),
            Self::MoveWithOnOff(cmd) => cmd.direction(),
            Self::StepWithOnOff(cmd) => cmd.direction(),
            Self::StopWithOnOff(cmd) => cmd.direction(),
            Self::MoveToClosestFrequency(cmd) => cmd.direction(),
        }
    }

    fn disable_default_response(&self) -> bool {
        match self {
            Self::MoveToLevel(cmd) => cmd.disable_default_response(),
            Self::Move(cmd) => cmd.disable_default_response(),
            Self::Step(cmd) => cmd.disable_default_response(),
            Self::Stop(cmd) => cmd.disable_default_response(),
            Self::MoveToLevelWithOnOff(cmd) => cmd.disable_default_response(),
            Self::MoveWithOnOff(cmd) => cmd.disable_default_response(),
            Self::StepWithOnOff(cmd) => cmd.disable_default_response(),
            Self::StopWithOnOff(cmd) => cmd.disable_default_response(),
            Self::MoveToClosestFrequency(cmd) => cmd.disable_default_response(),
        }
    }
}

impl ToLeStream for Command {
    type Iter = Iter;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::MoveToLevel(cmd) => Iter::MoveToLevel(cmd.to_le_stream()),
            Self::Move(cmd) => Iter::Move(cmd.to_le_stream()),
            Self::Step(cmd) => Iter::Step(cmd.to_le_stream()),
            Self::Stop(cmd) => Iter::Stop(cmd.to_le_stream()),
            Self::MoveToLevelWithOnOff(cmd) => Iter::MoveToLevelWithOnOff(cmd.to_le_stream()),
            Self::MoveWithOnOff(cmd) => Iter::MoveWithOnOff(cmd.to_le_stream()),
            Self::StepWithOnOff(cmd) => Iter::StepWithOnOff(cmd.to_le_stream()),
            Self::StopWithOnOff(cmd) => Iter::StopWithOnOff(cmd.to_le_stream()),
            Self::MoveToClosestFrequency(cmd) => Iter::MoveToClosestFrequency(cmd.to_le_stream()),
        }
    }
}

#[derive(Debug)]
pub enum Iter {
    MoveToLevel(<MoveToLevel as ToLeStream>::Iter),
    Move(<Move as ToLeStream>::Iter),
    Step(<Step as ToLeStream>::Iter),
    Stop(<Stop as ToLeStream>::Iter),
    MoveToLevelWithOnOff(<MoveToLevelWithOnOff as ToLeStream>::Iter),
    MoveWithOnOff(<MoveWithOnOff as ToLeStream>::Iter),
    StepWithOnOff(<StepWithOnOff as ToLeStream>::Iter),
    StopWithOnOff(<StopWithOnOff as ToLeStream>::Iter),
    MoveToClosestFrequency(<MoveToClosestFrequency as ToLeStream>::Iter),
}

impl Iterator for Iter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        #[expect(clippy::match_same_arms)]
        match self {
            Self::MoveToLevel(iter) => iter.next(),
            Self::Move(iter) => iter.next(),
            Self::Step(iter) => iter.next(),
            Self::Stop(iter) => iter.next(),
            Self::MoveToLevelWithOnOff(iter) => iter.next(),
            Self::MoveWithOnOff(iter) => iter.next(),
            Self::StepWithOnOff(iter) => iter.next(),
            Self::StopWithOnOff(iter) => iter.next(),
            Self::MoveToClosestFrequency(iter) => iter.next(),
        }
    }
}
