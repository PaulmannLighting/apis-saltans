use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Color mode of the device.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive)]
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
        Self::from_u8(value).ok_or(value)
    }
}
