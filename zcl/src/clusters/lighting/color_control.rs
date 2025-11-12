//! The `Color Control` cluster provides control over the color of a light source.

pub use self::color_information_attribute::ColorInformationAttribute;
pub use self::color_information_attribute::color_capabilities::ColorCapabilities;
pub use self::color_information_attribute::color_loop_direction::ColorLoopDirection;
pub use self::color_information_attribute::color_mode::ColorMode;
pub use self::color_information_attribute::drift_compensation::DriftCompensation;
pub use self::color_information_attribute::enhanced_color_mode::EnhancedColorMode;
pub use self::color_information_attribute::options::Options;
pub use self::color_information_attribute::startup_color_temperature::StartupColorTemperature;
pub use self::commands::{
    ColorLoopSet, EnhancedMoveHue, EnhancedMoveToHue, EnhancedMoveToHueAndSaturation,
    EnhancedStepHue, MoveColor, MoveColorTemperature, MoveHue, MoveSaturation, MoveToColor,
    MoveToColorTemperature, MoveToHue, MoveToHueAndSaturation, MoveToSaturation, StepColor,
    StepColorTemperature, StepHue, StepSaturation, StopMoveStep, color_loop_set, move_hue,
    move_saturation, move_to_hue, step_hue, step_saturation,
};

mod color_information_attribute;
mod commands;

const CLUSTER_ID: u16 = 0x0300;
