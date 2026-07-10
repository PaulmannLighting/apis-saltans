use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

/// Simple Descriptor application flags.
///
/// ZDP stores the application version in the high nibble and reserves the low
/// nibble.
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
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
    /// Return flags with the application version bits set to `version`.
    ///
    /// Only the low four bits of `version` are meaningful in the simple
    /// descriptor wire format.
    #[must_use]
    pub const fn with_version(self, version: u8) -> Self {
        Self(self.bits() | (version << Self::VERSION.bits().trailing_zeros()))
    }

    /// Return the application version stored in the high nibble.
    #[must_use]
    pub fn version(self) -> u8 {
        (self & Self::VERSION).bits() >> Self::VERSION.bits().trailing_zeros()
    }
}
