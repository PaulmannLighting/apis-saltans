use core::fmt::{self, Display};
use core::str::FromStr;

use bitflags::{bitflags, parser};
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
        let version = version << Self::VERSION.bits().trailing_zeros();

        Self((self.bits() & !Self::VERSION.bits()) | (version & Self::VERSION.bits()))
    }

    /// Return the application version stored in the high nibble.
    #[must_use]
    pub fn version(self) -> u8 {
        (self & Self::VERSION).bits() >> Self::VERSION.bits().trailing_zeros()
    }
}

impl Display for AppFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        parser::to_writer(self, f)
    }
}

impl FromStr for AppFlags {
    type Err = parser::ParseError;

    fn from_str(flags: &str) -> Result<Self, Self::Err> {
        parser::from_str(flags)
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::string::ToString;

    use super::AppFlags;

    const PARTIAL_VERSION_BITS: u8 = 0x10;
    const REPLACEMENT_VERSION: u8 = 0x05;
    const VERSION_WITH_HIGH_BITS: u8 = 0xF5;
    const VERSION_AND_RESERVED: &str = "VERSION | RESERVED";

    #[test]
    fn replaces_existing_version_and_preserves_reserved_bits() {
        let flags = (AppFlags::VERSION | AppFlags::RESERVED).with_version(REPLACEMENT_VERSION);

        assert_eq!(flags.version(), REPLACEMENT_VERSION);
        assert_eq!(flags & AppFlags::RESERVED, AppFlags::RESERVED);
    }

    #[test]
    fn ignores_version_bits_above_the_low_nibble() {
        let flags = AppFlags::empty().with_version(VERSION_WITH_HIGH_BITS);

        assert_eq!(flags.version(), REPLACEMENT_VERSION);
    }

    #[test]
    fn displays_named_flags() {
        assert_eq!(
            (AppFlags::VERSION | AppFlags::RESERVED).to_string(),
            VERSION_AND_RESERVED
        );
    }

    #[test]
    fn displays_partial_flags_as_hexadecimal_bits() {
        assert_eq!(
            AppFlags::from_bits_retain(PARTIAL_VERSION_BITS).to_string(),
            "0x10"
        );
    }

    #[test]
    fn parses_named_flags() {
        let parsed = VERSION_AND_RESERVED.parse::<AppFlags>();

        assert!(matches!(
            parsed,
            Ok(flags) if flags == AppFlags::VERSION | AppFlags::RESERVED
        ));
    }

    #[test]
    fn parses_hexadecimal_bits() {
        let parsed = "0x10".parse::<AppFlags>();

        assert!(matches!(
            parsed,
            Ok(flags) if flags == AppFlags::from_bits_retain(PARTIAL_VERSION_BITS)
        ));
    }

    #[test]
    fn display_and_parsing_round_trip_every_bit_pattern() {
        for bits in u8::MIN..=u8::MAX {
            let flags = AppFlags::from_bits_retain(bits);
            let parsed = flags.to_string().parse::<AppFlags>();

            assert!(matches!(parsed, Ok(parsed_flags) if parsed_flags == flags));
        }
    }

    #[test]
    fn rejects_unknown_flag_names() {
        assert!("UNKNOWN".parse::<AppFlags>().is_err());
    }
}
