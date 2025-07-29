use le_stream::derive::{FromLeStream, ToLeStream};

const NON_VALUE: u8 = 0xff;

/// The `8-bit unsigned integer` type, short `uint8`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Uint8(u8);

impl From<u8> for Uint8 {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<Uint8> for Option<u8> {
    fn from(value: Uint8) -> Self {
        if value.0 == NON_VALUE {
            None
        } else {
            Some(value.0)
        }
    }
}
