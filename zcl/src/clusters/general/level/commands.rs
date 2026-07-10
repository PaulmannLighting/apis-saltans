use zb_core::Cluster;

pub use self::r#move::Move;
pub use self::move_to_closest_frequency::MoveToClosestFrequency;
pub use self::move_to_level::MoveToLevel;
pub use self::move_to_level_with_on_off::MoveToLevelWithOnOff;
pub use self::move_with_on_off::MoveWithOnOff;
pub use self::step::Step;
pub use self::step_with_on_off::StepWithOnOff;
pub use self::stop::Stop;
pub use self::stop_with_on_off::StopWithOnOff;
use crate::macros::zcl_command_enum;

mod r#move;
mod move_to_closest_frequency;
mod move_to_level;
mod move_to_level_with_on_off;
mod move_with_on_off;
mod step;
mod step_with_on_off;
mod stop;
mod stop_with_on_off;

// Available Level cluster commands.
zcl_command_enum! {
    { Cluster::Level } => Level;
    MoveToLevel(MoveToLevel),
    Move(Move),
    Step(Step),
    Stop(Stop),
    MoveToLevelWithOnOff(MoveToLevelWithOnOff),
    MoveWithOnOff(MoveWithOnOff),
    StepWithOnOff(StepWithOnOff),
    StopWithOnOff(StopWithOnOff),
    MoveToClosestFrequency(MoveToClosestFrequency),
}
