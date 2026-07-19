use core::ops::RangeInclusive;
use core::str::FromStr;

use thiserror::Error;

/// A Zigbee application endpoint ID.
///
/// Application endpoints can be parsed from a decimal ID or a hexadecimal ID with a `0x` prefix.
/// Values outside `1..=240` are rejected.
#[cfg_attr(
    feature = "serde",
    expect(clippy::unsafe_derive_deserialize),
    derive(serde::Serialize, serde::Deserialize),
    serde(try_from = "u8", into = "u8")
)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
#[repr(transparent)]
pub struct Application(pub(super) u8);

impl Application {
    /// The minimum valid application endpoint ID.
    pub const MIN_ID: u8 = 1;

    /// The minimum valid application endpoint.
    pub const MIN: Self = Self(Self::MIN_ID);

    /// The maximum valid application endpoint ID.
    pub const MAX_ID: u8 = 240;

    /// The maximum valid application endpoint.
    pub const MAX: Self = Self(Self::MAX_ID);

    /// The valid application endpoint ID range.
    pub const RANGE: RangeInclusive<Self> = Self::MIN..=Self::MAX;

    /// Create a new `Application` endpoint ID if the given ID is valid.
    #[must_use]
    pub const fn new(id: u8) -> Option<Self> {
        if id >= Self::MIN_ID && id <= Self::MAX_ID {
            Some(Self(id))
        } else {
            None
        }
    }

    /// Create a new `Application` endpoint ID, clamping the given ID to the valid range.
    #[must_use]
    pub fn new_clamped(id: u8) -> Self {
        Self(id.clamp(Self::MIN_ID, Self::MAX_ID))
    }

    /// Create a new `Application` endpoint ID without checking validity.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the given ID is within the valid range (1..=240).
    #[expect(unsafe_code)]
    #[must_use]
    pub const unsafe fn new_unchecked(id: u8) -> Self {
        Self(id)
    }

    /// Return the raw application endpoint ID.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self.0
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::MIN
    }
}

impl_fmt_via_value!(Application, u8, |value| value.as_u8());

impl From<Application> for u8 {
    fn from(endpoint: Application) -> Self {
        endpoint.0
    }
}

impl FromStr for Application {
    type Err = ParseApplicationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::try_from(parse_application_id(value)?).map_err(|_| ParseApplicationError)
    }
}

impl TryFrom<u8> for Application {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(value)
    }
}

/// Error returned when parsing an unknown, malformed, or out-of-range application endpoint.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("invalid Zigbee application endpoint")]
pub struct ParseApplicationError;
fn parse_application_id(value: &str) -> Result<u8, ParseApplicationError> {
    value.strip_prefix("0x").map_or_else(
        || value.parse().map_err(|_| ParseApplicationError),
        |value| u8::from_str_radix(value, 16).map_err(|_| ParseApplicationError),
    )
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::string::ToString;

    use super::{Application, ParseApplicationError};

    const APPLICATION_ID: u8 = 10;

    #[test]
    fn parses_decimal_id() {
        assert_eq!(
            APPLICATION_ID.to_string().parse(),
            Ok(Application::new(APPLICATION_ID).unwrap())
        );
    }

    #[test]
    fn parses_hexadecimal_id() {
        assert_eq!(
            "0x0A".parse(),
            Ok(Application::new(APPLICATION_ID).unwrap())
        );
    }

    #[test]
    fn parses_range_boundaries() {
        assert_eq!(
            Application::MIN_ID.to_string().parse(),
            Ok(Application::MIN)
        );
        assert_eq!(
            Application::MAX_ID.to_string().parse(),
            Ok(Application::MAX)
        );
    }

    #[test]
    fn rejects_out_of_range_ids() {
        assert_eq!("0".parse::<Application>(), Err(ParseApplicationError));
        assert_eq!("241".parse::<Application>(), Err(ParseApplicationError));
    }

    #[test]
    fn rejects_unsupported_representations() {
        assert_eq!(
            "Application".parse::<Application>(),
            Err(ParseApplicationError)
        );
        assert_eq!("0X0A".parse::<Application>(), Err(ParseApplicationError));
    }
}
