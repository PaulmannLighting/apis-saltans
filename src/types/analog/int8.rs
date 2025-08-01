use le_stream::derive::{FromLeStream, ToLeStream};

/// The `8-bit signed integer` type, short `int8`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Int8(i8);

impl Int8 {
    /// The non-value.  See Table 2-11.
    pub const NON_VALUE: i8 = 0x80u8.cast_signed();

    /// Crate a new `Int8` from an `i8` value.
    #[must_use]
    pub const fn new(value: i8) -> Option<Self> {
        if value == Self::NON_VALUE {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Create a new `Int8` with the non-value.
    #[must_use]
    pub const fn non_value() -> Self {
        Self(Self::NON_VALUE)
    }
}

impl From<Int8> for Option<i8> {
    fn from(value: Int8) -> Self {
        if value.0 == Int8::NON_VALUE {
            None
        } else {
            Some(value.0)
        }
    }
}

impl TryFrom<i8> for Int8 {
    type Error = ();

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}
