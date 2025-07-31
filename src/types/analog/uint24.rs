use intx::U24;
use le_stream::derive::{FromLeStream, ToLeStream};

/// See Table 2-11.
const NON_VALUE: U24 = U24::MAX; // 0xffffff

/// The `24-bit unsigned integer` type, short `uint24`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Uint24(U24);

impl Uint24 {
    /// Crate a new `Uint24` from an `U24` value.
    #[must_use]
    pub fn new(value: U24) -> Option<Self> {
        if value == NON_VALUE {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Create a new `Uint24` with the non-value.
    #[must_use]
    pub const fn non_value(self) -> Self {
        Self(NON_VALUE)
    }
}

impl From<Uint24> for Option<U24> {
    fn from(value: Uint24) -> Self {
        if value.0 == NON_VALUE {
            None
        } else {
            Some(value.0)
        }
    }
}

impl From<Uint24> for Option<u32> {
    fn from(value: Uint24) -> Self {
        Option::<U24>::from(value).map(Into::into)
    }
}

impl TryFrom<U24> for Uint24 {
    type Error = ();

    fn try_from(value: U24) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}

impl TryFrom<u32> for Uint24 {
    type Error = Option<u32>;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        U24::try_from(value).map_or(Err(Some(value)), |u24| Self::new(u24).ok_or(None))
    }
}
