use core::fmt::{self, Display, LowerHex, UpperHex};
use core::str::FromStr;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::Endpoint;

/// Trait for types that belong to a Zigbee profile.
pub trait Profiled {
    /// The Zigbee profile.
    const PROFILE: Profile;
}

/// Defines the Zigbee Profile Identifiers as per the Zigbee specification.
///
/// Profiles can be parsed from their exact variant name, decimal identifier, or hexadecimal
/// identifier with a `0x` prefix.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, FromPrimitive)]
#[repr(u16)]
pub enum Profile {
    /// Profile Identifier for Zigbee Device Profile(ZDP).
    Network = 0x0000,

    /// Profile Identifier for Home Automation Profile.
    ZigbeeHomeAutomation = 0x0104,

    /// Profile Identifier for Building Automation Profile.
    BuildingAutomation = 0x0105,

    /// Profile Identifier for Remote Control Profile.
    RemoteControl = 0x0107,

    /// Profile Identifier for Health Care Profile.
    HealthCare = 0x0108,

    /// Profile Identifier for Smart Energy Profile.
    SmartEnergy = 0x0109,

    /// Profile Identifier for Light Link Profile.
    TouchLink = 0xC05E,
}

impl Profile {
    /// Returns the profile identifier as a `u16`.
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        self as u16
    }

    /// Return the endpoint used for profile-level broadcasts.
    ///
    /// The Zigbee Device Profile uses the data endpoint, while application
    /// profiles use the broadcast endpoint.
    #[must_use]
    pub const fn broadcast_endpoint(self) -> Endpoint {
        if matches!(self, Self::Network) {
            Endpoint::Data
        } else {
            Endpoint::Broadcast
        }
    }
}

impl FromStr for Profile {
    type Err = ParseProfileError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Some(profile) = profile_from_name(value) {
            return Ok(profile);
        }

        Self::try_from(parse_profile_identifier(value)?).map_err(|_| ParseProfileError)
    }
}

impl LowerHex for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        LowerHex::fmt(&self.as_u16(), f)
    }
}

impl UpperHex for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        UpperHex::fmt(&self.as_u16(), f)
    }
}

impl TryFrom<u16> for Profile {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value).ok_or(value)
    }
}

impl From<Profile> for u16 {
    fn from(profile: Profile) -> Self {
        profile.as_u16()
    }
}

/// Error returned when parsing an unknown or malformed Zigbee profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParseProfileError;

impl Display for ParseProfileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid Zigbee profile")
    }
}

impl core::error::Error for ParseProfileError {}

impl Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ({:#06X})", self, self.as_u16())
    }
}

fn profile_from_name(value: &str) -> Option<Profile> {
    match value {
        "Network" => Some(Profile::Network),
        "ZigbeeHomeAutomation" => Some(Profile::ZigbeeHomeAutomation),
        "BuildingAutomation" => Some(Profile::BuildingAutomation),
        "RemoteControl" => Some(Profile::RemoteControl),
        "HealthCare" => Some(Profile::HealthCare),
        "SmartEnergy" => Some(Profile::SmartEnergy),
        "TouchLink" => Some(Profile::TouchLink),
        _ => None,
    }
}

fn parse_profile_identifier(value: &str) -> Result<u16, ParseProfileError> {
    value.strip_prefix("0x").map_or_else(
        || value.parse().map_err(|_| ParseProfileError),
        |value| u16::from_str_radix(value, 16).map_err(|_| ParseProfileError),
    )
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::format;
    use alloc::string::ToString;

    use super::{ParseProfileError, Profile};

    const HOME_AUTOMATION_ID: u16 = 0x0104;
    const HOME_AUTOMATION_NAME: &str = "ZigbeeHomeAutomation";
    const HOME_AUTOMATION_DISPLAY: &str = "ZigbeeHomeAutomation (0x0104)";
    const TOUCH_LINK_LOWER_HEX: &str = "0xc05e";
    const TOUCH_LINK_UPPER_HEX: &str = "0xC05E";

    #[test]
    fn returns_numeric_identifier() {
        assert_eq!(Profile::ZigbeeHomeAutomation.as_u16(), HOME_AUTOMATION_ID);
    }

    #[test]
    fn displays_name_and_numeric_identifier() {
        assert_eq!(
            Profile::ZigbeeHomeAutomation.to_string(),
            HOME_AUTOMATION_DISPLAY
        );
    }

    #[test]
    fn formats_lower_hexadecimal_identifier() {
        assert_eq!(format!("{:#06x}", Profile::TouchLink), TOUCH_LINK_LOWER_HEX);
    }

    #[test]
    fn formats_upper_hexadecimal_identifier() {
        assert_eq!(format!("{:#06X}", Profile::TouchLink), TOUCH_LINK_UPPER_HEX);
    }

    #[test]
    fn parses_name() {
        assert_eq!(
            HOME_AUTOMATION_NAME.parse(),
            Ok(Profile::ZigbeeHomeAutomation)
        );
    }

    #[test]
    fn parses_decimal_identifier() {
        assert_eq!(
            HOME_AUTOMATION_ID.to_string().parse(),
            Ok(Profile::ZigbeeHomeAutomation)
        );
    }

    #[test]
    fn parses_hexadecimal_identifier() {
        assert_eq!("0x0104".parse(), Ok(Profile::ZigbeeHomeAutomation));
    }

    #[test]
    fn rejects_unknown_profile() {
        assert_eq!("Unknown".parse::<Profile>(), Err(ParseProfileError));
        assert_eq!("0xFFFF".parse::<Profile>(), Err(ParseProfileError));
    }

    #[test]
    fn rejects_unsupported_representations() {
        assert_eq!(
            HOME_AUTOMATION_DISPLAY.parse::<Profile>(),
            Err(ParseProfileError)
        );
        assert_eq!("0X0104".parse::<Profile>(), Err(ParseProfileError));
    }
}
