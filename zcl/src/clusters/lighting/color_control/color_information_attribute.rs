use le_stream::FromLeStreamTagged;
use zigbee::Parsable;
use zigbee::types::{String, Uint8, Uint16};

use self::color_capabilities::ColorCapabilities;
use self::options::Options;
use crate::clusters::lighting::color_control::{
    ColorLoopDirection, ColorMode, DriftCompensation, EnhancedColorMode, StartupColorTemperature,
};

pub mod color_capabilities;
pub mod color_loop_direction;
pub mod color_mode;
pub mod drift_compensation;
pub mod enhanced_color_mode;
pub mod options;
pub mod startup_color_temperature;

/// Color information attribute for the Color Control cluster.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
#[derive(FromLeStreamTagged)]
#[expect(variant_size_differences, clippy::large_enum_variant)]
pub enum ColorInformationAttribute {
    /// The current hue of the light.
    CurrentHue(Uint8) = 0x0000,
    /// The current saturation of the light.
    CurrentSaturation(Uint8) = 0x0001,
    /// Remaining time of the current command in 1/10 seconds.
    RemainingTime(Uint16) = 0x0002,
    /// The current X coordinate in the CIE 1931 color space.
    CurrentX(Uint16) = 0x0003,
    /// The current Y coordinate in the CIE 1931 color space.
    CurrentY(Uint16) = 0x0004,
    /// The drift compensation value for the light.
    DriftCompensation(Parsable<u16, DriftCompensation>) = 0x0005,
    /// The drift compensation text for the light.
    CompensationText(String<254>) = 0x0006,
    /// The color temperature of the light in mireds.
    ColorTemperature(Uint16) = 0x0007,
    /// The color mode of the light.
    ColorMode(Parsable<u8, ColorMode>) = 0x0008,
    /// Commissioning options.
    Options(Options) = 0x000f,
    /// The enhanced current hue of the light.
    EnhancedCurrentHue(Uint16) = 0x4000,
    /// The enhanced color mode of the light.
    EnhancedColorMode(Parsable<u16, EnhancedColorMode>) = 0x4001,
    /// Indicates whether the color loop is active.
    ColorLoopActive(Uint8) = 0x4002,
    /// The direction of the color loop.
    ColorLoopDirection(Parsable<u8, ColorLoopDirection>) = 0x4003,
    /// The time for one complete color loop cycle in seconds.
    ColorLoopTime(Uint16) = 0x4004,
    /// The start hue for the color loop in enhanced hue format.
    ColorLoopStartEnhancedHue(Uint16) = 0x4005,
    /// The stored enhanced hue value for the color loop.
    ColorLoopStoredEnhancedHue(Uint16) = 0x4006,
    /// The color capabilities of the light.
    ColorCapabilities(Parsable<u8, ColorCapabilities>) = 0x400a,
    /// The physical minimum color temperature in mireds.
    ColorTempPhysicalMin(Uint16) = 0x400b,
    /// The physical maximum color temperature in mireds.
    ColorTempPhysicalMax(Uint16) = 0x400c,
    /// The lower bound for the `ColorTemperature` in mireds.
    CoupleColorTempToLevelMin(Uint16) = 0x400d,
    /// The desired startup color temperature in mireds.
    StartUpColorTemperature(Parsable<u16, StartupColorTemperature>) = 0x4010,
}
