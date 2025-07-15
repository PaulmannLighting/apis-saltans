//! Lighting API.

pub use color_information_attribute::ColorInformationAttribute;
pub use color_loop_set::command::ColorLoopSet;
pub use enhanced_move_hue::EnhancedMoveHue;
pub use enhanced_move_to_hue::EnhancedMoveToHue;
pub use enhanced_move_to_hue_and_saturation::EnhancedMoveToHueAndSaturation;
pub use enhanced_step_hue::EnhancedStepHue;
pub use move_color::MoveColor;
pub use move_hue::command::MoveHue;
pub use move_saturation::command::MoveSaturation;
pub use move_to_color::MoveToColor;
pub use move_to_color_temperature::MoveToColorTemperature;
pub use move_to_hue::command::MoveToHue;
pub use step_hue::command::StepHue;
pub use step_saturation::command::StepSaturation;

use crate::zcl::Cluster;

mod color_information_attribute;
pub mod color_loop_set;
mod enhanced_move_hue;
mod enhanced_move_to_hue;
mod enhanced_move_to_hue_and_saturation;
mod enhanced_step_hue;
mod move_color;
pub mod move_hue;
pub mod move_saturation;
mod move_to_color;
mod move_to_color_temperature;
pub mod move_to_hue;
mod move_to_hue_and_saturation;
mod move_to_saturation;
mod step_color;
pub mod step_hue;
pub mod step_saturation;

const CLUSTER_ID: u16 = 0x0300;

/// Sealed trait for the Lighting cluster.
trait Lighting {}

impl<T> Cluster for T
where
    T: Lighting,
{
    const ID: u16 = CLUSTER_ID;
}
