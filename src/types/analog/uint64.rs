use le_stream::derive::{FromLeStream, ToLeStream};

/// See Table 2-11.
const NON_VALUE: u64 = 0xffff_ffff_ffff_ffff;

/// The `64-bit unsigned integer` type, short `uint64`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Uint64(u64);

impl Uint64 {
    /// Crate a new `Uint64` from an `u64` value.
    #[must_use]
    pub const fn new(value: u64) -> Option<Self> {
        if value == NON_VALUE {
            None
        } else {
            Some(Self(value))
        }
    }
}

impl From<Uint64> for Option<u64> {
    fn from(value: Uint64) -> Self {
        if value.0 == NON_VALUE {
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

impl TryFrom<Uint64> for u64 {
    type Error = Option<u64>;

    fn try_from(value: Uint64) -> Result<Self, Self::Error> {
        if value.0 == NON_VALUE {
            Err(None)
        } else {
            Ok(value.0)
        }
    }
}
