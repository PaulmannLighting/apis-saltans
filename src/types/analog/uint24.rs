use intx::U24;
use le_stream::derive::{FromLeStream, ToLeStream};

/// See Table 2-11.
const NON_VALUE: U24 = U24::MAX;

/// The `16-bit unsigned integer` type, short `uint16`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Uint24(U24);

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
