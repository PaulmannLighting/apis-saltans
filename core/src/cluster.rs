use core::fmt::{self, LowerHex, UpperHex};

use num_enum::{IntoPrimitive, TryFromPrimitive};
use thiserror::Error;

/// Trait to identify a Zigbee cluster.
pub trait ClusterSpecific<T = u16> {
    /// The cluster identifier.
    const ID: T;
}

impl<T> ClusterSpecific<u16> for T
where
    T: ClusterSpecific<Cluster>,
{
    const ID: u16 = T::ID.as_u16();
}

/// Known ZCL cluster identifiers defined in this crate's `clusters` module.
///
/// Clusters can be parsed from their exact variant name, decimal identifier, or hexadecimal
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
#[strum(parse_err_ty = ParseClusterError, parse_err_fn = parse_cluster_error)]
#[repr(u16)]
pub enum Cluster {
    /// Basic cluster.
    #[strum(
        to_string = "Basic (0x0000)",
        serialize = "Basic",
        serialize = "0",
        serialize = "0x0000"
    )]
    Basic = 0x0000,

    /// Power configuration cluster.
    #[strum(
        to_string = "PowerConfiguration (0x0001)",
        serialize = "PowerConfiguration",
        serialize = "1",
        serialize = "0x0001"
    )]
    PowerConfiguration = 0x0001,

    /// Device temperature configuration cluster.
    #[strum(
        to_string = "DeviceTemperatureConfiguration (0x0002)",
        serialize = "DeviceTemperatureConfiguration",
        serialize = "2",
        serialize = "0x0002"
    )]
    DeviceTemperatureConfiguration = 0x0002,

    /// Identify cluster.
    #[strum(
        to_string = "Identify (0x0003)",
        serialize = "Identify",
        serialize = "3",
        serialize = "0x0003"
    )]
    Identify = 0x0003,

    /// Groups cluster.
    #[strum(
        to_string = "Groups (0x0004)",
        serialize = "Groups",
        serialize = "4",
        serialize = "0x0004"
    )]
    Groups = 0x0004,

    /// Scenes cluster.
    #[strum(
        to_string = "Scenes (0x0005)",
        serialize = "Scenes",
        serialize = "5",
        serialize = "0x0005"
    )]
    Scenes = 0x0005,

    /// On/Off cluster.
    #[strum(
        to_string = "OnOff (0x0006)",
        serialize = "OnOff",
        serialize = "6",
        serialize = "0x0006"
    )]
    OnOff = 0x0006,

    /// Level control cluster.
    #[strum(
        to_string = "Level (0x0008)",
        serialize = "Level",
        serialize = "8",
        serialize = "0x0008"
    )]
    Level = 0x0008,

    /// Alarms cluster.
    #[strum(
        to_string = "Alarms (0x0009)",
        serialize = "Alarms",
        serialize = "9",
        serialize = "0x0009"
    )]
    Alarms = 0x0009,

    /// Time cluster.
    #[strum(
        to_string = "Time (0x000A)",
        serialize = "Time",
        serialize = "10",
        serialize = "0x000A",
        serialize = "0x000a"
    )]
    Time = 0x000A,

    /// OTA Upgrade cluster.
    #[strum(
        to_string = "OtaUpgrade (0x0019)",
        serialize = "OtaUpgrade",
        serialize = "25",
        serialize = "0x0019"
    )]
    OtaUpgrade = 0x0019,

    /// Color control cluster.
    #[strum(
        to_string = "ColorControl (0x0300)",
        serialize = "ColorControl",
        serialize = "768",
        serialize = "0x0300"
    )]
    ColorControl = 0x0300,

    /// Ballast configuration cluster.
    #[strum(
        to_string = "BallastConfiguration (0x0301)",
        serialize = "BallastConfiguration",
        serialize = "769",
        serialize = "0x0301"
    )]
    BallastConfiguration = 0x0301,

    /// Illuminance measurement cluster.
    #[strum(
        to_string = "IlluminanceMeasurement (0x0400)",
        serialize = "IlluminanceMeasurement",
        serialize = "1024",
        serialize = "0x0400"
    )]
    IlluminanceMeasurement = 0x0400,

    /// Illuminance level sensing cluster.
    #[strum(
        to_string = "IlluminanceLevelSensing (0x0401)",
        serialize = "IlluminanceLevelSensing",
        serialize = "1025",
        serialize = "0x0401"
    )]
    IlluminanceLevelSensing = 0x0401,

    /// Occupancy sensing cluster.
    #[strum(
        to_string = "OccupancySensing (0x0406)",
        serialize = "OccupancySensing",
        serialize = "1030",
        serialize = "0x0406"
    )]
    OccupancySensing = 0x0406,

    /// IAS Zone cluster.
    #[strum(
        to_string = "IasZone (0x0500)",
        serialize = "IasZone",
        serialize = "1280",
        serialize = "0x0500"
    )]
    IasZone = 0x0500,

    /// Keep-Alive cluster.
    #[strum(
        to_string = "KeepAlive (0x0025)",
        serialize = "KeepAlive",
        serialize = "37",
        serialize = "0x0025"
    )]
    KeepAlive = 0x0025,
}

impl Cluster {
    /// Returns the cluster ID as a `u16`.
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        self as u16
    }
}

impl LowerHex for Cluster {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        LowerHex::fmt(&self.as_u16(), f)
    }
}

impl UpperHex for Cluster {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        UpperHex::fmt(&self.as_u16(), f)
    }
}

/// Error returned when parsing an unknown or malformed Zigbee cluster.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("invalid Zigbee cluster")]
pub struct ParseClusterError;

const fn parse_cluster_error(_: &str) -> ParseClusterError {
    ParseClusterError
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::string::ToString;

    use strum::IntoEnumIterator;

    use super::{Cluster, ParseClusterError};

    const COLOR_CONTROL_ID: u16 = 0x0300;
    const COLOR_CONTROL_NAME: &str = "ColorControl";
    const COLOR_CONTROL_DISPLAY: &str = "ColorControl (0x0300)";

    #[test]
    fn returns_numeric_identifier() {
        assert_eq!(Cluster::ColorControl.as_u16(), COLOR_CONTROL_ID);
    }

    #[test]
    fn displays_name_and_numeric_identifier() {
        assert_eq!(Cluster::ColorControl.to_string(), COLOR_CONTROL_DISPLAY);
    }

    #[test]
    fn parses_name() {
        assert_eq!(COLOR_CONTROL_NAME.parse(), Ok(Cluster::ColorControl));
    }

    #[test]
    fn parses_decimal_identifier() {
        assert_eq!(
            COLOR_CONTROL_ID.to_string().parse(),
            Ok(Cluster::ColorControl)
        );
    }

    #[test]
    fn parses_hexadecimal_identifier() {
        assert_eq!("0x0300".parse(), Ok(Cluster::ColorControl));
    }

    #[test]
    fn display_and_parsing_round_trip() {
        for cluster in Cluster::iter() {
            assert_eq!(cluster.to_string().parse(), Ok(cluster));
        }
    }

    #[test]
    fn rejects_unknown_cluster() {
        assert_eq!("Unknown".parse::<Cluster>(), Err(ParseClusterError));
        assert_eq!("0xFFFF".parse::<Cluster>(), Err(ParseClusterError));
    }

    #[test]
    fn rejects_unsupported_representations() {
        assert_eq!("0X0300".parse::<Cluster>(), Err(ParseClusterError));
    }
}
