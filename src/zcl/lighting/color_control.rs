//! The `Color Control` cluster provides control over the color of a light source.

pub use color_information_attribute::ColorInformationAttribute;
pub use color_mode::ColorMode;
pub use commands::{
    ColorLoopSet, EnhancedMoveHue, EnhancedMoveToHue, EnhancedMoveToHueAndSaturation,
    EnhancedStepHue, MoveColor, MoveColorTemperature, MoveHue, MoveSaturation, MoveToColor,
    MoveToColorTemperature, MoveToHue, MoveToHueAndSaturation, MoveToSaturation, StepColor,
    StepColorTemperature, StepHue, StepSaturation, StopMoveStep, color_loop_set, move_hue,
    move_saturation, move_to_hue, step_hue, step_saturation,
};
pub use drift_compensation::DriftCompensation;
pub use options::Options;

mod color_information_attribute;
mod color_mode;
mod commands;
mod drift_compensation;
mod enhanced_color_mode;
mod options;

const CLUSTER_ID: u16 = 0x0300;
