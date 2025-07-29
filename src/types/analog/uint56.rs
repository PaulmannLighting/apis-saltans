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
