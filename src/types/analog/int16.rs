use le_stream::derive::{FromLeStream, ToLeStream};

/// See Table 2-11.
#[allow(overflowing_literals)]
const NON_VALUE: i16 = 0x8000;

/// The `16-bit signed integer` type, short `int16`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Int16(i16);

impl Int16 {
    /// Crate a new `Int16` from an `i16` value.
    #[must_use]
    pub const fn new(value: i16) -> Option<Self> {
        if value == NON_VALUE {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Create a new `Int16` with the non-value.
    #[must_use]
    pub const fn non_value(self) -> Self {
        Self(NON_VALUE)
    }
}

impl From<Int16> for Option<i16> {
    fn from(value: Int16) -> Self {
        if value.0 == NON_VALUE {
            None
        } else {
            Some(value.0)
        }
    }
}

impl TryFrom<i16> for Int16 {
    type Error = ();

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}
