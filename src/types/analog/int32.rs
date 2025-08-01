use le_stream::derive::{FromLeStream, ToLeStream};

/// The `32-bit signed integer` type, short `int32`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Int32(i32);

impl Int32 {
    /// The non-value. See Table 2-11.
    pub const NON_VALUE: i32 = 0x8000_0000u32.cast_signed();

    /// Crate a new `Int32` from an `i32` value.
    #[must_use]
    pub const fn new(value: i32) -> Option<Self> {
        if value == Self::NON_VALUE {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Create a new `Int32` with the non-value.
    #[must_use]
    pub const fn non_value() -> Self {
        Self(Self::NON_VALUE)
    }
}

impl From<Int32> for Option<i32> {
    fn from(value: Int32) -> Self {
        if value.0 == Int32::NON_VALUE {
            None
        } else {
            Some(value.0)
        }
    }
}

impl TryFrom<i32> for Int32 {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}
