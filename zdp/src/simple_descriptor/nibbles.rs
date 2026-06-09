use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

/// Nibbles for version and reserved bits.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct Nibbles(u8);

bitflags! {
    impl Nibbles: u8 {
        /// Version nibble.
        const VERSION = 0b1111_0000;

        /// Reserved nibble.
        const RESERVED = 0b0000_1111;
    }
}

impl Nibbles {
    /// Return the version number.
    #[must_use]
    pub fn version(self) -> u8 {
        (self & Self::VERSION).bits() >> Self::VERSION.bits().trailing_zeros()
    }
}
