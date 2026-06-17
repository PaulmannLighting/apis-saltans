use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

/// Application flags for the version and reserved bits.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct AppFlags(u8);

bitflags! {
    impl AppFlags: u8 {
        /// Version nibble.
        const VERSION = 0b1111_0000;

        /// Reserved nibble.
        const RESERVED = 0b0000_1111;
    }
}

impl AppFlags {
    /// Return the version number.
    #[must_use]
    pub fn version(self) -> u8 {
        (self & Self::VERSION).bits() >> Self::VERSION.bits().trailing_zeros()
    }
}
