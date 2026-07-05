//! Attributes of the Color Control cluster.

use apis_saltans_core::ClusterId;
use apis_saltans_core::types::{String, Type, Uint8, Uint16};

use crate::macros::zcl_attributes;

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
    DriftCompensation = 0x0005: Type { R },
    /// The drift compensation text for the light.
    CompensationText = 0x0006: String<254> { R },
    /// The color temperature of the light in mireds.
    ColorTemperature = 0x0007: Uint16 { R },
    /// The color mode of the light.
    ColorMode = 0x0008: Type { R },
    /// Commissioning options.
    Options = 0x000f: Type { R, W },
    /// The enhanced current hue of the light.
    EnhancedCurrentHue = 0x4000: Uint16 { R },
    /// The enhanced color mode of the light.
    EnhancedColorMode = 0x4001: Type { R },
    /// Indicates whether the color loop is active.
    ColorLoopActive = 0x4002: Uint8 { R },
    /// The direction of the color loop.
    ColorLoopDirection = 0x4003: Type { R },
    /// The time for one complete color loop cycle in seconds.
    ColorLoopTime = 0x4004: Uint16 { R },
    /// The start hue for the color loop in enhanced hue format.
    ColorLoopStartEnhancedHue = 0x4005: Uint16 { R },
    /// The stored enhanced hue value for the color loop.
    ColorLoopStoredEnhancedHue = 0x4006: Uint16 { R },
    /// The color capabilities of the light.
    ColorCapabilities = 0x400a: Type { R },
    /// The physical minimum color temperature in mireds.
    ColorTempPhysicalMin = 0x400b: Uint16 { R },
    /// The physical maximum color temperature in mireds.
    ColorTempPhysicalMax = 0x400c: Uint16 { R },
    /// The lower bound for the `ColorTemperature` in mireds.
    CoupleColorTempToLevelMin = 0x400d: Uint16 { R },
    /// The desired startup color temperature in mireds.
    StartUpColorTemperature = 0x4010: Type { R, W },
}
