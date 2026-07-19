use core::fmt::{self, LowerHex, UpperHex};

use num_enum::{IntoPrimitive, TryFromPrimitive};
use thiserror::Error;

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
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    IntoPrimitive,
    Ord,
    PartialEq,
    PartialOrd,
    strum::Display,
    strum::EnumString,
    TryFromPrimitive,
)]
#[cfg_attr(test, derive(strum::EnumIter))]
#[num_enum(error_type(name = u16, constructor = core::convert::identity))]
#[strum(parse_err_ty = ParseProfileError, parse_err_fn = parse_profile_error)]
#[repr(u16)]
pub enum Profile {
    /// Profile Identifier for Zigbee Device Profile(ZDP).
    #[strum(
        to_string = "Network (0x0000)",
        serialize = "Network",
        serialize = "0",
        serialize = "0x0000"
    )]
    Network = 0x0000,

    /// Profile Identifier for Home Automation Profile.
    #[strum(
        to_string = "ZigbeeHomeAutomation (0x0104)",
        serialize = "ZigbeeHomeAutomation",
        serialize = "260",
        serialize = "0x0104"
    )]
    ZigbeeHomeAutomation = 0x0104,

    /// Profile Identifier for Building Automation Profile.
    #[strum(
        to_string = "BuildingAutomation (0x0105)",
        serialize = "BuildingAutomation",
        serialize = "261",
        serialize = "0x0105"
    )]
    BuildingAutomation = 0x0105,

    /// Profile Identifier for Remote Control Profile.
    #[strum(
        to_string = "RemoteControl (0x0107)",
        serialize = "RemoteControl",
        serialize = "263",
        serialize = "0x0107"
    )]
    RemoteControl = 0x0107,

    /// Profile Identifier for Health Care Profile.
    #[strum(
        to_string = "HealthCare (0x0108)",
        serialize = "HealthCare",
        serialize = "264",
        serialize = "0x0108"
    )]
    HealthCare = 0x0108,

    /// Profile Identifier for Smart Energy Profile.
    #[strum(
        to_string = "SmartEnergy (0x0109)",
        serialize = "SmartEnergy",
        serialize = "265",
        serialize = "0x0109"
    )]
    SmartEnergy = 0x0109,

    /// Profile Identifier for Light Link Profile.
    #[strum(
        to_string = "TouchLink (0xC05E)",
        serialize = "TouchLink",
        serialize = "49246",
        serialize = "0xC05E",
        serialize = "0xc05e"
    )]
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

/// Error returned when parsing an unknown or malformed Zigbee profile.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("invalid Zigbee profile")]
pub struct ParseProfileError;

const fn parse_profile_error(_: &str) -> ParseProfileError {
    ParseProfileError
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::format;
    use alloc::string::ToString;

    use strum::IntoEnumIterator;

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
    fn display_and_parsing_round_trip() {
        for profile in Profile::iter() {
            assert_eq!(profile.to_string().parse(), Ok(profile));
        }
    }

    #[test]
    fn rejects_unknown_profile() {
        assert_eq!("Unknown".parse::<Profile>(), Err(ParseProfileError));
        assert_eq!("0xFFFF".parse::<Profile>(), Err(ParseProfileError));
    }

    #[test]
    fn rejects_unsupported_representations() {
        assert_eq!("0X0104".parse::<Profile>(), Err(ParseProfileError));
    }
}
