use std::iter::Chain;

use le_stream::{FromLeStream, ToLeStream};
use repr_discriminant::repr_discriminant;

use crate::types::U24;
use crate::util::DeviceTemperatureConfigurationAttributeIterator;

/// Attributes for the Device Temperature Configuration cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr_discriminant(u16, id)]
pub enum Attribute {
    // Device Temperature Information.
    /// Current temperature in degrees Celsius.
    CurrentTemperature(i16) = 0x0000,
    /// Minimum temperature experienced in degrees Celsius.
    MinTempExperienced(i16) = 0x0001,
    /// Maximum temperature experienced in degrees Celsius.
    MaxTempExperienced(i16) = 0x0002,
    /// Total time the temperature was above the maximum threshold in hours.
    OverTempTotalDwell(u16) = 0x0003,
    // Device Temperature Settings.
    /// Alarms mask for device temperature.
    DeviceTempAlarmMask(u8) = 0x0010,
    /// Low temperature threshold in degrees Celsius.
    LowTempThreshold(i16) = 0x0011,
    /// High temperature threshold in degrees Celsius.
    HighTempThreshold(i16) = 0x0012,
    /// Low temperature dwell trip point in seconds.
    LowTempDwellTripPoint(U24) = 0x0013,
    /// High temperature dwell trip point in seconds.
    HighTempDwellTripPoint(U24) = 0x0014,
}

impl FromLeStream for Attribute {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        match u16::from_le_stream(&mut bytes)? {
            0x0000 => i16::from_le_stream(bytes).map(Self::CurrentTemperature),
            0x0001 => i16::from_le_stream(bytes).map(Self::MinTempExperienced),
            0x0002 => i16::from_le_stream(bytes).map(Self::MaxTempExperienced),
            0x0003 => u16::from_le_stream(bytes).map(Self::OverTempTotalDwell),
            0x0010 => u8::from_le_stream(bytes).map(Self::DeviceTempAlarmMask),
            0x0011 => i16::from_le_stream(bytes).map(Self::LowTempThreshold),
            0x0012 => i16::from_le_stream(bytes).map(Self::HighTempThreshold),
            0x0013 => U24::from_le_stream(bytes).map(Self::LowTempDwellTripPoint),
            0x0014 => U24::from_le_stream(bytes).map(Self::HighTempDwellTripPoint),
            _ => None,
        }
    }
}

impl ToLeStream for Attribute {
    type Iter = Chain<<u16 as ToLeStream>::Iter, DeviceTemperatureConfigurationAttributeIterator>;

    fn to_le_stream(self) -> Self::Iter {
        let id = self.id();
        let payload_iter: DeviceTemperatureConfigurationAttributeIterator = match self {
            Self::CurrentTemperature(temp)
            | Self::MinTempExperienced(temp)
            | Self::MaxTempExperienced(temp)
            | Self::LowTempThreshold(temp)
            | Self::HighTempThreshold(temp) => temp.into(),
            Self::OverTempTotalDwell(hours) => hours.into(),
            Self::DeviceTempAlarmMask(mask) => mask.into(),
            Self::LowTempDwellTripPoint(seconds) | Self::HighTempDwellTripPoint(seconds) => {
                seconds.into()
            }
        };
        id.to_le_stream().chain(payload_iter)
    }
}
