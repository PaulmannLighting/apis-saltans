//! The `Color Control` cluster provides control over the color of a light source.

pub use color_capabilities::ColorCapabilities;
pub use color_information_attribute::ColorInformationAttribute;
pub use color_loop_direction::ColorLoopDirection;
pub use color_mode::ColorMode;
pub use commands::{
    ColorLoopSet, EnhancedMoveHue, EnhancedMoveToHue, EnhancedMoveToHueAndSaturation,
    EnhancedStepHue, MoveColor, MoveColorTemperature, MoveHue, MoveSaturation, MoveToColor,
    MoveToColorTemperature, MoveToHue, MoveToHueAndSaturation, MoveToSaturation, StepColor,
    StepColorTemperature, StepHue, StepSaturation, StopMoveStep, color_loop_set, move_hue,
    move_saturation, move_to_hue, step_hue, step_saturation,
};
pub use drift_compensation::DriftCompensation;
pub use enhanced_color_mode::EnhancedColorMode;
pub use options::Options;
pub use startup_color_temperature::StartupColorTemperature;

mod color_capabilities;
mod color_information_attribute;
mod color_loop_direction;
mod color_mode;
mod commands;
mod drift_compensation;
mod enhanced_color_mode;
mod options;
mod startup_color_temperature;

const CLUSTER_ID: u16 = 0x0300;
