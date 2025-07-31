use le_stream::derive::{FromLeStream, ToLeStream};

/// The `64-bit unsigned integer` type, short `uint64`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Uint64(u64);

impl Uint64 {
    /// The non-value. See Table 2-11.
    pub const NON_VALUE: u64 = 0xffff_ffff_ffff_ffff;

    /// Crate a new `Uint64` from an `u64` value.
    #[must_use]
    pub const fn new(value: u64) -> Option<Self> {
        if value == Self::NON_VALUE {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Create a new `Uint64` with the non-value.
    #[must_use]
    pub const fn non_value() -> Self {
        Self(Self::NON_VALUE)
    }
}

impl From<Uint64> for Option<u64> {
    fn from(value: Uint64) -> Self {
        if value.0 == Uint64::NON_VALUE {
            None
        } else {
            Some(value.0)
        }
    }
}

impl TryFrom<u64> for Uint64 {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}
