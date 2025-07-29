use le_stream::derive::{FromLeStream, ToLeStream};

/// The `8-bit unsigned integer` type, short `uint8`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Uint8(u8);

impl Uint8 {
    /// The non-value. See Table 2-11.
    pub const NON_VALUE: u8 = 0xff;

    /// Crate a new `Uint8` from an `u8` value.
    #[must_use]
    pub const fn new(value: u8) -> Option<Self> {
        if value == Self::NON_VALUE {
            None
        } else {
            Some(Self(value))
        }
    }
}

impl From<Uint8> for Option<u8> {
    fn from(value: Uint8) -> Self {
        if value.0 == Uint8::NON_VALUE {
            None
        } else {
            Some(value.0)
        }
    }
}
