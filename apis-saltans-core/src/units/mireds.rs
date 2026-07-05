use le_stream::{FromLeStream, ToLeStream};

use crate::types::{Type, Uint16};

/// Represents a color temperature in mireds (micro reciprocal degrees).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Mireds(u16);

impl Mireds {
    /// Minimum value for mireds.
    pub const MIN: u16 = 0x0000;
    /// Maximum value for mireds.
    pub const MAX: u16 = 0xffef;
}

impl From<Mireds> for u16 {
    fn from(value: Mireds) -> Self {
        value.0
    }
}

impl From<Mireds> for Uint16 {
    fn from(value: Mireds) -> Self {
        Self::new(value.0)
    }
}

impl From<Mireds> for Type {
    fn from(value: Mireds) -> Self {
        Self::Uint16(value.into())
    }
}

impl TryFrom<u16> for Mireds {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(value)
        }
    }
}

impl TryFrom<Type> for Mireds {
    type Error = Type;

    fn try_from(value: Type) -> Result<Self, Self::Error> {
        if let Type::Uint16(value) = value {
            Option::<u16>::from(value)
                .and_then(|value| Self::try_from(value).ok())
                .ok_or(Type::Uint16(value))
        } else {
            Err(value)
        }
    }
}
