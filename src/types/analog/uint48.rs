use intx::U48;
use le_stream::derive::{FromLeStream, ToLeStream};

/// See Table 2-11.
const NON_VALUE: U48 = U48::MAX; // 0xffff_ffff_ffff

/// The `48-bit unsigned integer` type, short `uint48`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Uint48(U48);

impl Uint48 {
    /// Crate a new `Uint48` from an `U48` value.
    #[must_use]
    pub fn new(value: U48) -> Option<Self> {
        if value == NON_VALUE {
            None
        } else {
            Some(Self(value))
        }
    }
}

impl From<Uint48> for Option<U48> {
    fn from(value: Uint48) -> Self {
        if value.0 == NON_VALUE {
            None
        } else {
            Some(value.0)
        }
    }
}

impl From<Uint48> for Option<u64> {
    fn from(value: Uint48) -> Self {
        Option::<U48>::from(value).map(Into::into)
    }
}

impl TryFrom<U48> for Uint48 {
    type Error = ();

    fn try_from(value: U48) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}

impl TryFrom<u64> for Uint48 {
    type Error = Option<u64>;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        U48::try_from(value).map_or(Err(Some(value)), |u48| Self::new(u48).ok_or(None))
    }
}
