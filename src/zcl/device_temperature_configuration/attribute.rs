use core::iter::Chain;

use intx::U24;
use le_stream::{FromLeStream, FromLeStreamTagged, ToLeStream};
use repr_discriminant::repr_discriminant;

use crate::util::DeviceTemperatureConfigurationAttributeIterator;
use crate::zcl::device_temperature_configuration::DeviceTempAlarmMask;

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
    DeviceTempAlarmMask(DeviceTempAlarmMask) = 0x0010,
    /// Low temperature threshold in degrees Celsius.
    LowTempThreshold(i16) = 0x0011,
    /// High temperature threshold in degrees Celsius.
    HighTempThreshold(i16) = 0x0012,
    /// Low temperature dwell trip point in seconds.
    LowTempDwellTripPoint(U24) = 0x0013,
    /// High temperature dwell trip point in seconds.
    HighTempDwellTripPoint(U24) = 0x0014,
}

impl FromLeStreamTagged for Attribute {
    type Tag = u16;

    fn from_le_stream_tagged<T>(tag: Self::Tag, bytes: T) -> Result<Option<Self>, Self::Tag>
    where
        T: Iterator<Item = u8>,
    {
        match tag {
            0x0000 => Ok(i16::from_le_stream(bytes).map(Self::CurrentTemperature)),
            0x0001 => Ok(i16::from_le_stream(bytes).map(Self::MinTempExperienced)),
            0x0002 => Ok(i16::from_le_stream(bytes).map(Self::MaxTempExperienced)),
            0x0003 => Ok(u16::from_le_stream(bytes).map(Self::OverTempTotalDwell)),
            0x0010 => Ok(DeviceTempAlarmMask::from_le_stream(bytes).map(Self::DeviceTempAlarmMask)),
            0x0011 => Ok(i16::from_le_stream(bytes).map(Self::LowTempThreshold)),
            0x0012 => Ok(i16::from_le_stream(bytes).map(Self::HighTempThreshold)),
            0x0013 => Ok(U24::from_le_stream(bytes).map(Self::LowTempDwellTripPoint)),
            0x0014 => Ok(U24::from_le_stream(bytes).map(Self::HighTempDwellTripPoint)),
            unknown => Err(unknown),
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
