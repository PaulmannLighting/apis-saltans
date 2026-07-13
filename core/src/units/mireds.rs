use le_stream::{FromLeStream, ToLeStream};

use crate::types::{Type, Uint16};

/// A color temperature expressed in mireds (micro reciprocal degrees).
///
/// One mired is one million reciprocal kelvins (`1_000_000 / K`), so higher
/// mired values represent lower color temperatures. Valid protocol values range
/// from [`Mireds::MIN`] through [`Mireds::MAX`], inclusive.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Mireds(u16);

impl Mireds {
    /// The minimum valid raw mired value.
    pub const MIN_VALUE: u16 = 0x0000;

    /// The minimum valid color temperature value.
    pub const MIN: Self = Self(Self::MIN_VALUE);

    /// The maximum valid raw mired value.
    pub const MAX_VALUE: u16 = 0xffef;

    /// The maximum valid color temperature value.
    pub const MAX: Self = Self(Self::MAX_VALUE);
}

impl Mireds {
    /// Creates a color temperature from a raw mired value.
    ///
    /// # Errors
    ///
    /// Returns the supplied value if it is greater than [`Mireds::MAX_VALUE`].
    pub const fn try_new(mireds: u16) -> Result<Self, u16> {
        if mireds > Self::MAX_VALUE {
            Err(mireds)
        } else {
            Ok(Self(mireds))
        }
    }
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
        if value > Self::MAX_VALUE {
            Err(value)
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<Uint16> for Mireds {
    type Error = Uint16;

    fn try_from(value: Uint16) -> Result<Self, Self::Error> {
        Self::try_from(value.into_inner()).map_err(|_| value)
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
