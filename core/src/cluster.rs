use core::fmt::{self, Display, LowerHex, UpperHex};
use core::str::FromStr;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

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
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive)]
#[repr(u16)]
pub enum Cluster {
    /// Basic cluster.
    Basic = 0x0000,

    /// Power configuration cluster.
    PowerConfiguration = 0x0001,

    /// Device temperature configuration cluster.
    DeviceTemperatureConfiguration = 0x0002,

    /// Identify cluster.
    Identify = 0x0003,

    /// Groups cluster.
    Groups = 0x0004,

    /// Scenes cluster.
    Scenes = 0x0005,

    /// On/Off cluster.
    OnOff = 0x0006,

    /// Level control cluster.
    Level = 0x0008,

    /// Alarms cluster.
    Alarms = 0x0009,

    /// Time cluster.
    Time = 0x000A,

    /// Color control cluster.
    ColorControl = 0x0300,

    /// Ballast configuration cluster.
    BallastConfiguration = 0x0301,

    /// Illuminance measurement cluster.
    IlluminanceMeasurement = 0x0400,

    /// Illuminance level sensing cluster.
    IlluminanceLevelSensing = 0x0401,

    /// Occupancy sensing cluster.
    OccupancySensing = 0x0406,

    /// IAS Zone cluster.
    IasZone = 0x0500,
}

impl Cluster {
    /// Returns the cluster ID as a `u16`.
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        self as u16
    }
}

impl FromStr for Cluster {
    type Err = ParseClusterError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Some(cluster) = cluster_from_name(value) {
            return Ok(cluster);
        }

        Self::try_from(parse_cluster_identifier(value)?).map_err(|_| ParseClusterError)
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

impl From<Cluster> for u16 {
    fn from(cluster_id: Cluster) -> Self {
        cluster_id.as_u16()
    }
}

impl TryFrom<u16> for Cluster {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value).ok_or(value)
    }
}

/// Error returned when parsing an unknown or malformed Zigbee cluster.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParseClusterError;

impl Display for ParseClusterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid Zigbee cluster")
    }
}

impl core::error::Error for ParseClusterError {}

impl Display for Cluster {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ({:#06X})", self, self.as_u16())
    }
}

fn cluster_from_name(value: &str) -> Option<Cluster> {
    match value {
        "Basic" => Some(Cluster::Basic),
        "PowerConfiguration" => Some(Cluster::PowerConfiguration),
        "DeviceTemperatureConfiguration" => Some(Cluster::DeviceTemperatureConfiguration),
        "Identify" => Some(Cluster::Identify),
        "Groups" => Some(Cluster::Groups),
        "Scenes" => Some(Cluster::Scenes),
        "OnOff" => Some(Cluster::OnOff),
        "Level" => Some(Cluster::Level),
        "Alarms" => Some(Cluster::Alarms),
        "Time" => Some(Cluster::Time),
        "ColorControl" => Some(Cluster::ColorControl),
        "BallastConfiguration" => Some(Cluster::BallastConfiguration),
        "IlluminanceMeasurement" => Some(Cluster::IlluminanceMeasurement),
        "IlluminanceLevelSensing" => Some(Cluster::IlluminanceLevelSensing),
        "OccupancySensing" => Some(Cluster::OccupancySensing),
        "IasZone" => Some(Cluster::IasZone),
        _ => None,
    }
}

fn parse_cluster_identifier(value: &str) -> Result<u16, ParseClusterError> {
    value.strip_prefix("0x").map_or_else(
        || value.parse().map_err(|_| ParseClusterError),
        |value| u16::from_str_radix(value, 16).map_err(|_| ParseClusterError),
    )
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::string::ToString;

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
    fn rejects_unknown_cluster() {
        assert_eq!("Unknown".parse::<Cluster>(), Err(ParseClusterError));
        assert_eq!("0xFFFF".parse::<Cluster>(), Err(ParseClusterError));
    }

    #[test]
    fn rejects_unsupported_representations() {
        assert_eq!(
            COLOR_CONTROL_DISPLAY.parse::<Cluster>(),
            Err(ParseClusterError)
        );
        assert_eq!("0X0300".parse::<Cluster>(), Err(ParseClusterError));
    }
}
