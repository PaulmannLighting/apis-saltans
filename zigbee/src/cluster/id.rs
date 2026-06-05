/// Known ZCL cluster identifiers defined in this crate's `clusters` module.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
pub enum Id {
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
