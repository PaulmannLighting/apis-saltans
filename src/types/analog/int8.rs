use le_stream::derive::{FromLeStream, ToLeStream};

/// See Table 2-11.
#[allow(overflowing_literals)]
const NON_VALUE: i8 = 0x80;

/// The `8-bit signed integer` type, short `int8`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Int8(i8);

impl Int8 {
    /// Crate a new `Int8` from an `i8` value.
    #[must_use]
    pub const fn new(value: i8) -> Option<Self> {
        if value == NON_VALUE {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Create a new `Int8` with the non-value.
    #[must_use]
    pub const fn non_value(self) -> Self {
        Self(NON_VALUE)
    }
}

impl From<Int8> for Option<i8> {
    fn from(value: Int8) -> Self {
        if value.0 == NON_VALUE {
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
