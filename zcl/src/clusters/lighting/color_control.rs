//! The `Color Control` cluster provides control over the color of a light source.

pub use self::attributes::{Id, Readable, Reportable, SendReport, Writable};
pub use self::commands::{
    ColorLoopSet, Command, EnhancedMoveHue, EnhancedMoveToHue, EnhancedMoveToHueAndSaturation,
    EnhancedStepHue, MoveColor, MoveColorTemperature, MoveHue, MoveSaturation, MoveToColor,
    MoveToColorTemperature, MoveToHue, MoveToHueAndSaturation, MoveToSaturation, StepColor,
    StepColorTemperature, StepHue, StepSaturation, StopMoveStep, color_loop_set, move_hue,
    move_saturation, move_to_hue, step_hue, step_saturation,
};

mod attributes;
mod commands;
