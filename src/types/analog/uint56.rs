use intx::U56;
use le_stream::derive::{FromLeStream, ToLeStream};

/// See Table 2-11.
const NON_VALUE: U56 = U56::MAX; // 0xff_ffff_ffff_ffff

/// The `56-bit unsigned integer` type, short `uint56`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Uint56(U56);

impl Uint56 {
    /// Crate a new `Uint56` from an `U56` value.
    #[must_use]
    pub fn new(value: U56) -> Option<Self> {
        if value == NON_VALUE {
            None
        } else {
            Some(Self(value))
        }
    }
}

impl From<Uint56> for Option<U56> {
    fn from(value: Uint56) -> Self {
        if value.0 == NON_VALUE {
            None
        } else {
            Some(value.0)
        }
    }
}

impl From<Uint56> for Option<u64> {
    fn from(value: Uint56) -> Self {
        Option::<U56>::from(value).map(Into::into)
    }
}

impl TryFrom<U56> for Uint56 {
    type Error = ();

    fn try_from(value: U56) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}

impl TryFrom<u64> for Uint56 {
    type Error = Option<u64>;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        U56::try_from(value).map_or(Err(Some(value)), |u56| Self::new(u56).ok_or(None))
    }
}
