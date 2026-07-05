//! Attributes of the Color Control cluster.

use apis_saltans_core::ClusterId;
use apis_saltans_core::types::{String, Uint8, Uint16};
use apis_saltans_core::units::Mireds;

pub use self::color_capabilities::ColorCapabilities;
pub use self::color_loop_direction::ColorLoopDirection;
pub use self::color_mode::ColorMode;
pub use self::drift_compensation::DriftCompensation;
pub use self::enhanced_color_mode::EnhancedColorMode;
pub use self::options::Options;
pub use self::startup_color_temperature::StartupColorTemperature;
use crate::macros::zcl_attributes;

mod color_capabilities;
mod color_loop_direction;
mod color_mode;
mod drift_compensation;
mod enhanced_color_mode;
mod options;
mod startup_color_temperature;

zcl_attributes! {
    cluster: ClusterId::ColorControl;

    /// The current hue of the light.
    CurrentHue = 0x0000: Uint8 { R },
    /// The current saturation of the light.
    CurrentSaturation = 0x0001: Uint8 { R },
    /// Remaining time of the current command in 1/10 seconds.
    RemainingTime = 0x0002: Uint16 { R },
    /// The current X coordinate in the CIE 1931 color space.
    CurrentX = 0x0003: Uint16 { R },
    /// The current Y coordinate in the CIE 1931 color space.
    CurrentY = 0x0004: Uint16 { R },
    /// The drift compensation value for the light.
    DriftCompensation = 0x0005: DriftCompensation { R },
    /// The drift compensation text for the light.
    CompensationText = 0x0006: String<254> { R },
    /// The color temperature of the light in mireds.
    ColorTemperature = 0x0007: Mireds { R },
    /// The color mode of the light.
    ColorMode = 0x0008: ColorMode { R },
    /// Commissioning options.
    Options = 0x000f: Options { R, W },
    /// The enhanced current hue of the light.
    EnhancedCurrentHue = 0x4000: Uint16 { R },
    /// The enhanced color mode of the light.
    EnhancedColorMode = 0x4001: EnhancedColorMode { R },
    /// Indicates whether the color loop is active.
    ColorLoopActive = 0x4002: Uint8 { R },
    /// The direction of the color loop.
    ColorLoopDirection = 0x4003: ColorLoopDirection { R },
    /// The time for one complete color loop cycle in seconds.
    ColorLoopTime = 0x4004: Uint16 { R },
    /// The start hue for the color loop in enhanced hue format.
    ColorLoopStartEnhancedHue = 0x4005: Uint16 { R },
    /// The stored enhanced hue value for the color loop.
    ColorLoopStoredEnhancedHue = 0x4006: Uint16 { R },
    /// The color capabilities of the light.
    ColorCapabilities = 0x400a: ColorCapabilities { R },
    /// The physical minimum color temperature in mireds.
    ColorTempPhysicalMin = 0x400b: Uint16 { R },
    /// The physical maximum color temperature in mireds.
    ColorTempPhysicalMax = 0x400c: Uint16 { R },
    /// The lower bound for the `ColorTemperature` in mireds.
    CoupleColorTempToLevelMin = 0x400d: Uint16 { R },
    /// The desired startup color temperature in mireds.
    StartUpColorTemperature = 0x4010: StartupColorTemperature { R, W },
}
