use le_stream::derive::{FromLeStream, ToLeStream};

/// See Table 2-11.
#[allow(overflowing_literals)]
const NON_VALUE: i32 = 0x8000_0000;

/// The `32-bit signed integer` type, short `int24`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Int32(i32);

impl Int32 {
    /// Crate a new `Int32` from an `i32` value.
    #[must_use]
    pub const fn new(value: i32) -> Option<Self> {
        if value == NON_VALUE {
            None
        } else {
            Some(Self(value))
        }
    }
}

impl From<Int32> for Option<i32> {
    fn from(value: Int32) -> Self {
        if value.0 == NON_VALUE {
            None
        } else {
            Some(value.0)
        }
    }
}
