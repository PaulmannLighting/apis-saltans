use le_stream::derive::{FromLeStream, ToLeStream};

/// See Table 2-11.
#[allow(clippy::cast_possible_wrap)]
const NON_VALUE: i8 = 0x80u8 as i8;

/// The `8-bit signed integer` type, short `int8`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Int8(i8);

impl From<Int8> for Option<i8> {
    fn from(value: Int8) -> Self {
        if value.0 == NON_VALUE {
            None
        } else {
            Some(value.0)
        }
    }
}
