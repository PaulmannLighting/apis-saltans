use core::iter::Chain;

use intx::U24;
use le_stream::ToLeStream;
use le_stream::derive::FromLeStreamTagged;
use repr_discriminant::repr_discriminant;

use super::temp_threshold::TempThreshold;
use super::{DeviceTempAlarmMask, Temperature};

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
    type Iter = Chain<<u16 as ToLeStream>::Iter, iterator::Attribute>;

    fn to_le_stream(self) -> Self::Iter {
        let id = self.id();
        let payload_iter: iterator::Attribute = match self {
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

/// Iterator for `Attribute` payloads.
mod iterator {
    use intx::U24;
    use le_stream::ToLeStream;

    use crate::zcl::device_temperature_configuration::{
        DeviceTempAlarmMask, TempThreshold, Temperature,
    };

    pub enum Attribute {
        Temperature(<Temperature as ToLeStream>::Iter),
        U16(<u16 as ToLeStream>::Iter),
        DeviceTempAlarmMask(<DeviceTempAlarmMask as ToLeStream>::Iter),
        TempThreshold(<TempThreshold as ToLeStream>::Iter),
        U24(<U24 as ToLeStream>::Iter),
    }

    impl Iterator for Attribute {
        type Item = u8;

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Self::U16(iter) | Self::Temperature(iter) | Self::TempThreshold(iter) => {
                    iter.next()
                }
                Self::DeviceTempAlarmMask(iter) => iter.next(),
                Self::U24(iter) => iter.next(),
            }
        }
    }

    impl From<Temperature> for Attribute {
        fn from(value: Temperature) -> Self {
            Self::Temperature(value.to_le_stream())
        }
    }

    impl From<u16> for Attribute {
        fn from(value: u16) -> Self {
            Self::U16(value.to_le_stream())
        }
    }

    impl From<DeviceTempAlarmMask> for Attribute {
        fn from(value: DeviceTempAlarmMask) -> Self {
            Self::DeviceTempAlarmMask(value.to_le_stream())
        }
    }

    impl From<TempThreshold> for Attribute {
        fn from(value: TempThreshold) -> Self {
            Self::TempThreshold(value.to_le_stream())
        }
    }

    impl From<U24> for Attribute {
        fn from(value: U24) -> Self {
            Self::U24(value.to_le_stream())
        }
    }
}
