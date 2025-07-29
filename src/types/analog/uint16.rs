use le_stream::derive::{FromLeStream, ToLeStream};

/// See Table 2-11.
const NON_VALUE: u16 = 0xffff;

/// The `16-bit unsigned integer` type, short `uint16`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Uint16(u16);

impl Uint16 {
    /// Crate a new `Uint16` from an `u16` value.
    #[must_use]
    pub const fn new(value: u16) -> Option<Self> {
        if value == NON_VALUE {
            None
        } else {
            Some(Self(value))
        }
    }
}

impl From<Uint16> for Option<u16> {
    fn from(value: Uint16) -> Self {
        if value.0 == NON_VALUE {
            None
        } else {
            Some(value.0)
        }
    }
}
