use intx::I56;
use le_stream::derive::{FromLeStream, ToLeStream};

/// See Table 2-11.
const NON_VALUE: [u8; 7] = [0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // big-endian representation of 0x80_0000_0000_0000

/// The `56-bit signed integer` type, short `int56`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Int56(I56);

impl Int56 {
    /// Crate a new `Int56` from an `I56` value.
    #[must_use]
    pub fn new(value: I56) -> Option<Self> {
        if value == I56::from_be_bytes(NON_VALUE) {
            None
        } else {
            Some(Self(value))
        }
    }
}

impl From<Int56> for Option<I56> {
    fn from(value: Int56) -> Self {
        if value.0 == I56::from_be_bytes(NON_VALUE) {
            None
        } else {
            Some(value.0)
        }
    }
}

impl From<Int56> for Option<i64> {
    fn from(value: Int56) -> Self {
        Option::<I56>::from(value).map(Into::into)
    }
}
