use core::iter::Chain;

use intx::U24;
use le_stream::ToLeStream;
use le_stream::derive::FromLeStreamTagged;
use repr_discriminant::repr_discriminant;

use super::temp_threshold::TempThreshold;
use super::{DeviceTempAlarmMask, Temperature};
use crate::util::DeviceTemperatureConfigurationAttributeIterator;

/// Attributes for the Device Temperature Configuration cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr_discriminant(u16, id)]
#[derive(FromLeStreamTagged)]
pub enum Attribute {
    // Device Temperature Information.
    /// Current temperature in degrees Celsius.
    CurrentTemperature(Temperature) = 0x0000,
    /// Minimum temperature experienced in degrees Celsius.
    MinTempExperienced(Temperature) = 0x0001,
    /// Maximum temperature experienced in degrees Celsius.
    MaxTempExperienced(Temperature) = 0x0002,
    /// Total time the temperature was above the maximum threshold in hours.
    OverTempTotalDwell(u16) = 0x0003,
    // Device Temperature Settings.
    /// Alarms mask for device temperature.
    DeviceTempAlarmMask(DeviceTempAlarmMask) = 0x0010,
    /// Low temperature threshold in degrees Celsius.
    LowTempThreshold(TempThreshold) = 0x0011,
    /// High temperature threshold in degrees Celsius.
    HighTempThreshold(TempThreshold) = 0x0012,
    /// Low temperature dwell trip point in seconds.
    LowTempDwellTripPoint(U24) = 0x0013,
    /// High temperature dwell trip point in seconds.
    HighTempDwellTripPoint(U24) = 0x0014,
}

impl ToLeStream for Attribute {
    type Iter = Chain<<u16 as ToLeStream>::Iter, DeviceTemperatureConfigurationAttributeIterator>;

    fn to_le_stream(self) -> Self::Iter {
        let id = self.id();
        let payload_iter: DeviceTemperatureConfigurationAttributeIterator = match self {
            Self::CurrentTemperature(temp)
            | Self::MinTempExperienced(temp)
            | Self::MaxTempExperienced(temp) => temp.into(),
            Self::LowTempThreshold(thresh) | Self::HighTempThreshold(thresh) => thresh.into(),
            Self::OverTempTotalDwell(hours) => hours.into(),
            Self::DeviceTempAlarmMask(mask) => mask.into(),
            Self::LowTempDwellTripPoint(seconds) | Self::HighTempDwellTripPoint(seconds) => {
                seconds.into()
            }
        };
        id.to_le_stream().chain(payload_iter)
    }
}
