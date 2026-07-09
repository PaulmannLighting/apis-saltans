//! Attributes of the Color Control cluster.

use apis_saltans_core::Cluster;
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
    cluster: Cluster::ColorControl;

    /// The current hue of the light.
    CurrentHue = 0x0000: Uint8 { R },
    /// The current saturation of the light.
    CurrentSaturation = 0x0001: Uint8 { R, P, S },
    /// Remaining time of the current command in 1/10 seconds.
    RemainingTime = 0x0002: Uint16 { R },
    /// The current X coordinate in the CIE 1931 color space.
    CurrentX = 0x0003: Uint16 { R, P, S },
    /// The current Y coordinate in the CIE 1931 color space.
    CurrentY = 0x0004: Uint16 { R, P, S },
    /// The drift compensation value for the light.
    DriftCompensation = 0x0005: DriftCompensation { R },
    /// The drift compensation text for the light.
    CompensationText = 0x0006: String<254> { R },
    /// The color temperature of the light in mireds.
    ColorTemperature = 0x0007: Mireds { R, P, S },
    /// The color mode of the light.
    ColorMode = 0x0008: ColorMode { R },
    /// Commissioning options.
    Options = 0x000f: Options { R, W },
    /// Number of color primaries implemented by the device.
    NumberOfPrimaries = 0x0010: Uint8 { R },
    /// X chromaticity value of the first primary.
    Primary1X = 0x0011: Uint16 { R },
    /// Y chromaticity value of the first primary.
    Primary1Y = 0x0012: Uint16 { R },
    /// Maximum intensity of the first primary.
    Primary1Intensity = 0x0013: Uint8 { R },
    /// X chromaticity value of the second primary.
    Primary2X = 0x0015: Uint16 { R },
    /// Y chromaticity value of the second primary.
    Primary2Y = 0x0016: Uint16 { R },
    /// Maximum intensity of the second primary.
    Primary2Intensity = 0x0017: Uint8 { R },
    /// X chromaticity value of the third primary.
    Primary3X = 0x0019: Uint16 { R },
    /// Y chromaticity value of the third primary.
    Primary3Y = 0x001a: Uint16 { R },
    /// Maximum intensity of the third primary.
    Primary3Intensity = 0x001b: Uint8 { R },
    /// X chromaticity value of the fourth primary.
    Primary4X = 0x0020: Uint16 { R },
    /// Y chromaticity value of the fourth primary.
    Primary4Y = 0x0021: Uint16 { R },
    /// Maximum intensity of the fourth primary.
    Primary4Intensity = 0x0022: Uint8 { R },
    /// X chromaticity value of the fifth primary.
    Primary5X = 0x0024: Uint16 { R },
    /// Y chromaticity value of the fifth primary.
    Primary5Y = 0x0025: Uint16 { R },
    /// Maximum intensity of the fifth primary.
    Primary5Intensity = 0x0026: Uint8 { R },
    /// X chromaticity value of the sixth primary.
    Primary6X = 0x0028: Uint16 { R },
    /// Y chromaticity value of the sixth primary.
    Primary6Y = 0x0029: Uint16 { R },
    /// Maximum intensity of the sixth primary.
    Primary6Intensity = 0x002a: Uint8 { R },
    /// X chromaticity value of the white point.
    WhitePointX = 0x0030: Uint16 { R, W },
    /// Y chromaticity value of the white point.
    WhitePointY = 0x0031: Uint16 { R, W },
    /// X chromaticity value of the red color point.
    ColorPointRX = 0x0032: Uint16 { R, W },
    /// Y chromaticity value of the red color point.
    ColorPointRY = 0x0033: Uint16 { R, W },
    /// Relative intensity of the red color point.
    ColorPointRIntensity = 0x0034: Uint8 { R, W },
    /// X chromaticity value of the green color point.
    ColorPointGX = 0x0036: Uint16 { R, W },
    /// Y chromaticity value of the green color point.
    ColorPointGY = 0x0037: Uint16 { R, W },
    /// Relative intensity of the green color point.
    ColorPointGIntensity = 0x0038: Uint8 { R, W },
    /// X chromaticity value of the blue color point.
    ColorPointBX = 0x003a: Uint16 { R, W },
    /// Y chromaticity value of the blue color point.
    ColorPointBY = 0x003b: Uint16 { R, W },
    /// Relative intensity of the blue color point.
    ColorPointBIntensity = 0x003c: Uint8 { R, W },
    /// The enhanced current hue of the light.
    EnhancedCurrentHue = 0x4000: Uint16 { R, S },
    /// The enhanced color mode of the light.
    EnhancedColorMode = 0x4001: EnhancedColorMode { R },
    /// Indicates whether the color loop is active.
    ColorLoopActive = 0x4002: Uint8 { R, S },
    /// The direction of the color loop.
    ColorLoopDirection = 0x4003: ColorLoopDirection { R, S },
    /// The time for one complete color loop cycle in seconds.
    ColorLoopTime = 0x4004: Uint16 { R, S },
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
