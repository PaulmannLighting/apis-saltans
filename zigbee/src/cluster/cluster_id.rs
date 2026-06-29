use core::fmt::{self, Display, LowerHex, UpperHex};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Known ZCL cluster identifiers defined in this crate's `clusters` module.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive)]
#[repr(u16)]
pub enum ClusterId {
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

    /// Illuminance measurement cluster.
    IlluminanceMeasurement = 0x0400,

    /// Illuminance level sensing cluster.
    IlluminanceLevelSensing = 0x0401,
}

impl ClusterId {
    /// Returns the cluster ID as a `u16`.
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        self as u16
    }
}

impl Display for ClusterId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ({:#06X})", self, self.as_u16())
    }
}

impl LowerHex for ClusterId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        LowerHex::fmt(&self.as_u16(), f)
    }
}

impl UpperHex for ClusterId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        UpperHex::fmt(&self.as_u16(), f)
    }
}

impl From<ClusterId> for u16 {
    fn from(cluster_id: ClusterId) -> Self {
        cluster_id.as_u16()
    }
}

impl TryFrom<u16> for ClusterId {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value).ok_or(value)
    }
}
