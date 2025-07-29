use intx::U40;
use le_stream::derive::{FromLeStream, ToLeStream};

/// See Table 2-11.
const NON_VALUE: U40 = U40::MAX; // 0xff_ffff_ffff

/// The `40-bit unsigned integer` type, short `uint40`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Uint40(U40);

impl From<Uint40> for Option<U40> {
    fn from(value: Uint40) -> Self {
        if value.0 == NON_VALUE {
            None
        } else {
            Some(value.0)
        }
    }
}

impl From<Uint40> for Option<u64> {
    fn from(value: Uint40) -> Self {
        Option::<U40>::from(value).map(Into::into)
    }
}
