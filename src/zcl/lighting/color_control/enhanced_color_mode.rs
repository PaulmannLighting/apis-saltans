/// Enhanced color modes.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum EnhancedColorMode {
    /// Current hue and saturation.
    CurrentHueAndSaturation = 0x00,
    /// Current X and Y coordinates.
    CurrentXAndY = 0x01,
    /// Color temperature in mireds.
    ColorTemperature = 0x02,
    /// Enhanced current hue and saturation.
    EnhancedCurrentHueAndSaturation = 0x03,
}

impl TryFrom<u8> for EnhancedColorMode {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::CurrentHueAndSaturation),
            0x01 => Ok(Self::CurrentXAndY),
            0x02 => Ok(Self::ColorTemperature),
            0x03 => Ok(Self::EnhancedCurrentHueAndSaturation),
            other => Err(other),
        }
    }
}
