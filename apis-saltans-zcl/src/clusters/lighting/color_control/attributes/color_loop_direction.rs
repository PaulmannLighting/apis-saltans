use apis_saltans_core::types::{Type, Uint8};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Direction of the color loop.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum ColorLoopDirection {
    /// Increment `EnhancedCurrentHue`.
    Increment = 0x00,
    /// Decrement `EnhancedCurrentHue`.
    Decrement = 0x01,
}

impl TryFrom<u8> for ColorLoopDirection {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}

impl From<ColorLoopDirection> for Type {
    fn from(value: ColorLoopDirection) -> Self {
        Self::Enum8(Uint8::new(value as u8))
    }
}

impl TryFrom<Uint8> for ColorLoopDirection {
    type Error = Uint8;

    fn try_from(value: Uint8) -> Result<Self, Self::Error> {
        Self::try_from(value.into_inner()).map_err(|_| value)
    }
}

impl TryFrom<Type> for ColorLoopDirection {
    type Error = Type;

    fn try_from(value: Type) -> Result<Self, Self::Error> {
        if let Type::Enum8(value) = value {
            Self::try_from(value).map_err(Type::Enum8)
        } else {
            Err(value)
        }
    }
}
