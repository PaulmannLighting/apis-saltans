use num_derive::FromPrimitive;

/// Color information attribute for the Color Control cluster.
///
/// TODO: Add respective associated data.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromPrimitive)]
#[repr(u16)]
pub enum ColorInformationAttribute {
    CurrentHue = 0x0000,
    CurrentSaturation = 0x0001,
    RemainingTime = 0x0002,
    CurrentX = 0x0003,
    CurrentY = 0x0004,
    DriftCompensation = 0x0005,
    CompensationText = 0x0006,
    ColorTemperature = 0x0007,
    ColorMode = 0x0008,
    EnhancedCurrentHue = 0x4000,
    EnhancedColorMode = 0x4001,
    ColorLoopActive = 0x4002,
    ColorLoopDirection = 0x4003,
    ColorLoopTime = 0x4004,
    ColorLoopStartEnhancedHue = 0x4005,
    ColorLoopStoredEnhancedHue = 0x4006,
    ColorCapabilities = 0x400A,
    ColorTempPhysicalMinMireds = 0x400B,
    ColorTempPhysicalMaxMireds = 0x400C,
}
