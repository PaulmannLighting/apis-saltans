use num_derive::FromPrimitive;

/// Color information attribute for the Color Control cluster.
///
/// TODO: Add respective associated data.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromPrimitive)]
#[repr(u16)]
pub enum ColorInformationAttribute {
    /// The current hue of the light.
    CurrentHue = 0x0000,
    /// The current saturation of the light.
    CurrentSaturation = 0x0001,
    /// Remaining time of the current command in 1/10 seconds.
    RemainingTime = 0x0002,
    /// The current X coordinate in the CIE 1931 color space.
    CurrentX = 0x0003,
    /// The current Y coordinate in the CIE 1931 color space.
    CurrentY = 0x0004,
    /// The drift compensation value for the light.
    DriftCompensation = 0x0005,
    /// The drift compensation text for the light.
    CompensationText = 0x0006,
    /// The color temperature of the light in mireds.
    ColorTemperature = 0x0007,
    /// The color mode of the light.
    ColorMode = 0x0008,
    /// The enhanced current hue of the light.
    EnhancedCurrentHue = 0x4000,
    /// The enhanced color mode of the light.
    EnhancedColorMode = 0x4001,
    /// Indicates whether the color loop is active.
    ColorLoopActive = 0x4002,
    /// The direction of the color loop.
    ColorLoopDirection = 0x4003,
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
