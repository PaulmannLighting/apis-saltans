/// Color mode of the device.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum ColorMode {
    /// Current hue and saturation.
    CurrentHueAndSaturation = 0x00,
    /// Current X and Y coordinates.
    CurrentXAndY = 0x01,
    /// Color temperature in mireds.
    ColorTemperature = 0x02,
}

impl TryFrom<u8> for ColorMode {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::CurrentHueAndSaturation),
            0x01 => Ok(Self::CurrentXAndY),
            0x02 => Ok(Self::ColorTemperature),
            other => Err(other),
        }
    }
}
