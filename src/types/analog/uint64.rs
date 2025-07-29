use le_stream::derive::{FromLeStream, ToLeStream};

/// See Table 2-11.
const NON_VALUE: u64 = 0xffff_ffff_ffff_ffff;

/// The `64-bit unsigned integer` type, short `uint64`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Uint64(u64);

impl From<Uint64> for Option<u64> {
    fn from(value: Uint64) -> Self {
        if value.0 == NON_VALUE {
            None
        } else {
            Some(value.0)
        }
    }
}
