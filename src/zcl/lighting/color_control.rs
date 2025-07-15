//! The `Color Control` cluster provides control over the color of a light source.

pub use color_information_attribute::ColorInformationAttribute;
pub use color_loop_set::command::ColorLoopSet;
pub use drift_compensation::DriftCompensation;
pub use enhanced_move_hue::EnhancedMoveHue;
pub use enhanced_move_to_hue::EnhancedMoveToHue;
pub use enhanced_move_to_hue_and_saturation::EnhancedMoveToHueAndSaturation;
pub use enhanced_step_hue::EnhancedStepHue;
pub use move_color::MoveColor;
pub use move_color_temperature::MoveColorTemperature;
pub use move_hue::command::MoveHue;
pub use move_saturation::command::MoveSaturation;
pub use move_to_color::MoveToColor;
pub use move_to_color_temperature::MoveToColorTemperature;
pub use move_to_hue::command::MoveToHue;
pub use move_to_hue_and_saturation::MoveToHueAndSaturation;
pub use move_to_saturation::MoveToSaturation;
pub use step_color::StepColor;
pub use step_color_temperature::StepColorTemperature;
pub use step_hue::command::StepHue;
pub use step_saturation::command::StepSaturation;
pub use stop_move_step::StopMoveStep;

mod color_information_attribute;
pub mod color_loop_set;
mod drift_compensation;
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

const CLUSTER_ID: u16 = 0x0300;
