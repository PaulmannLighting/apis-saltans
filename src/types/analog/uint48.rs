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
