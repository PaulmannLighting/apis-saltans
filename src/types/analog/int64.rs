use le_stream::derive::{FromLeStream, ToLeStream};

/// The `64-bit signed integer` type, short `int64`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Int64(i64);

impl Int64 {
    /// The non-value. See Table 2-11.
    pub const NON_VALUE: i64 = 0x8000_0000_0000_0000u64.cast_signed();

    /// Crate a new `Int64` from an `i64` value.
    #[must_use]
    pub const fn new(value: i64) -> Option<Self> {
        if value == Self::NON_VALUE {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Create a new `Int64` with the non-value.
    #[must_use]
    pub const fn non_value() -> Self {
        Self(Self::NON_VALUE)
    }
}

impl From<Int64> for Option<i64> {
    fn from(value: Int64) -> Self {
        if value == Int64::non_value() {
            None
        } else {
            Some(value.0)
        }
    }
}

impl TryFrom<i64> for Int64 {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}
