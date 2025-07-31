use le_stream::derive::{FromLeStream, ToLeStream};

/// The `32-bit unsigned integer` type, short `uint32`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Uint32(u32);

impl Uint32 {
    /// The non-value. See Table 2-11.
    pub const NON_VALUE: u32 = 0xffff;

    /// Crate a new `Uint32` from an `u32` value.
    #[must_use]
    pub const fn new(value: u32) -> Option<Self> {
        if value == Self::NON_VALUE {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Create a new `Uint32` with the non-value.
    #[must_use]
    pub const fn non_value() -> Self {
        Self(Self::NON_VALUE)
    }
}

impl From<Uint32> for Option<u32> {
    fn from(value: Uint32) -> Self {
        if value.0 == Uint32::NON_VALUE {
            None
        } else {
            Some(value.0)
        }
    }
}

impl TryFrom<u32> for Uint32 {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}
