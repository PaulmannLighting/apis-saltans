use le_stream::derive::{FromLeStream, ToLeStream};

/// See Table 2-11.
#[allow(overflowing_literals)]
const NON_VALUE: i64 = 0x8000_0000_0000_0000;

/// The `64-bit signed integer` type, short `int64`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Int64(i64);

impl Int64 {
    /// Crate a new `Int64` from an `i64` value.
    #[must_use]
    pub const fn new(value: i64) -> Option<Self> {
        if value == NON_VALUE {
            None
        } else {
            Some(Self(value))
        }
    }
}

impl From<Int64> for Option<i64> {
    fn from(value: Int64) -> Self {
        if value.0 == NON_VALUE {
            None
        } else {
            Some(value.0)
        }
    }
}
