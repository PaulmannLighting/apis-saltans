use num_enum::{IntoPrimitive, TryFromPrimitive};
use zb_core::types::{Enum8, Type, Uint8};

/// Enhanced color modes.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Eq, Hash, IntoPrimitive, Ord, PartialEq, PartialOrd, TryFromPrimitive,
)]
#[num_enum(error_type(name = u8, constructor = core::convert::identity))]
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

impl zb_core::TypeId for EnhancedColorMode {
    const ID: u8 = <Enum8 as zb_core::TypeId>::ID;
}

impl From<EnhancedColorMode> for Type {
    fn from(value: EnhancedColorMode) -> Self {
        Self::Enum8(Enum8::new(Uint8::new(value as u8)))
    }
}

impl TryFrom<Uint8> for EnhancedColorMode {
    type Error = Uint8;

    fn try_from(value: Uint8) -> Result<Self, Self::Error> {
        Self::try_from(value.into_inner()).map_err(|_| value)
    }
}

impl TryFrom<Type> for EnhancedColorMode {
    type Error = Type;

    fn try_from(value: Type) -> Result<Self, Self::Error> {
        if let Type::Enum8(value) = value {
            Self::try_from(value.into_inner()).map_err(|_| Type::Enum8(value))
        } else {
            Err(value)
        }
    }
}
