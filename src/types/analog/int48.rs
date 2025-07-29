use intx::I48;
use le_stream::derive::{FromLeStream, ToLeStream};

/// See Table 2-11.
const NON_VALUE: [u8; 6] = [0x80, 0x00, 0x00, 0x00, 0x00, 0x00]; // big-endian representation of 0x8000_0000_0000

/// The `48-bit signed integer` type, short `int48`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Int48(I48);

impl Int48 {
    /// Crate a new `Int48` from an `I48` value.
    #[must_use]
    pub fn new(value: I48) -> Option<Self> {
        if value == I48::from_be_bytes(NON_VALUE) {
            None
        } else {
            Some(Self(value))
        }
    }
}

impl From<Int48> for Option<I48> {
    fn from(value: Int48) -> Self {
        if value.0 == I48::from_be_bytes(NON_VALUE) {
            None
        } else {
            Some(value.0)
        }
    }
}

impl From<Int48> for Option<i64> {
    fn from(value: Int48) -> Self {
        Option::<I48>::from(value).map(Into::into)
    }
}
