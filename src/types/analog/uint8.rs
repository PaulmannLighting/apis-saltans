use le_stream::derive::{FromLeStream, ToLeStream};

/// See Table 2-11.
const NON_VALUE: u8 = 0xff;

/// The `8-bit unsigned integer` type, short `uint8`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Uint8(u8);

impl Uint8 {
    /// Crate a new `Uint8` from an `u8` value.
    #[must_use]
    pub const fn new(value: u8) -> Option<Self> {
        if value == NON_VALUE {
            None
        } else {
            Some(Self(value))
        }
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
