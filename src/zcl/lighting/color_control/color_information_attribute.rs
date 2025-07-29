use le_stream::derive::FromLeStreamTagged;
use maybe_color_loop_direction::MaybeColorLoopDirection;
use maybe_color_mode::MaybeColorMode;
use maybe_drift_compensation::MaybeDriftCompensation;
use maybe_enhanced_color_mode::MaybeEnhancedColorMode;

use super::options::Options;
use crate::types::{String, Uint8, Uint16};

mod maybe_color_loop_direction;
mod maybe_color_mode;
mod maybe_drift_compensation;
mod maybe_enhanced_color_mode;

/// Color information attribute for the Color Control cluster.
///
/// TODO: Add respective associated data.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
#[derive(FromLeStreamTagged)]
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
    DriftCompensation(MaybeDriftCompensation) = 0x0005,
    /// The drift compensation text for the light.
    CompensationText(String) = 0x0006,
    /// The color temperature of the light in mireds.
    ColorTemperature(Uint16) = 0x0007,
    /// The color mode of the light.
    ColorMode(MaybeColorMode) = 0x0008,
    /// Commissioning options.
    Options(Options) = 0x000f,
    /// The enhanced current hue of the light.
    EnhancedCurrentHue(Uint16) = 0x4000,
    /// The enhanced color mode of the light.
    EnhancedColorMode(MaybeEnhancedColorMode) = 0x4001,
    /// Indicates whether the color loop is active.
    ColorLoopActive(Uint8) = 0x4002,
    /// The direction of the color loop.
    ColorLoopDirection(MaybeColorLoopDirection) = 0x4003,
    /// The time for one complete color loop cycle in seconds.
    ColorLoopTime = 0x4004,
    /// The start hue for the color loop in enhanced hue format.
    ColorLoopStartEnhancedHue = 0x4005,
    /// The stored enhanced hue value for the color loop.
    ColorLoopStoredEnhancedHue = 0x4006,
    /// The color capabilities of the light.
    ColorCapabilities = 0x400A,
    /// The physical minimum color temperature in mireds.
    ColorTempPhysicalMin = 0x400B,
    /// The physical maximum color temperature in mireds.
    ColorTempPhysicalMax = 0x400C,
}
