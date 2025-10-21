use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Ballast Configuration attribute.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromPrimitive)]
#[repr(u16)]
pub enum BallastConfigurationAttribute {
    /// Ballast information.
    BallastInformation = 0x0000,
    /// Ballast settings.
    BallastSettings = 0x0001,
    /// Lamp information.
    LampInformation = 0x0002,
    /// Lamp settings.
    LampSettings = 0x0003,
}

impl TryFrom<u16> for BallastConfigurationAttribute {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value).ok_or(value)
    }
}
