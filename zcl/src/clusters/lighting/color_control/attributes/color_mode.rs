use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use zb_core::types::{Enum8, Type, Uint8};

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

impl zb_core::TypeId for ColorMode {
    const ID: u8 = <Enum8 as zb_core::TypeId>::ID;
}

impl TryFrom<u8> for ColorMode {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}

impl From<ColorMode> for Type {
    fn from(value: ColorMode) -> Self {
        Self::Enum8(Enum8::new(Uint8::new(value as u8)))
    }
}

impl TryFrom<Uint8> for ColorMode {
    type Error = Uint8;

    fn try_from(value: Uint8) -> Result<Self, Self::Error> {
        Self::try_from(value.into_inner()).map_err(|_| value)
    }
}

impl TryFrom<Type> for ColorMode {
    type Error = Type;

    fn try_from(value: Type) -> Result<Self, Self::Error> {
        if let Type::Enum8(value) = value {
            Self::try_from(value.into_inner()).map_err(|value| Type::Enum8(value.into()))
        } else {
            Err(value)
        }
    }
}
