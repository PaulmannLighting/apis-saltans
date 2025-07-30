use core::iter::Chain;

use le_stream::ToLeStream;
use le_stream::derive::FromLeStreamTagged;
use repr_discriminant::ReprDiscriminant;

use super::{DeviceTempAlarmMask, Temperature};
use crate::types::{Uint16, Uint24};

/// Attributes for the Device Temperature Configuration cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
#[derive(ReprDiscriminant, FromLeStreamTagged)]
pub enum Attribute {
    // Device Temperature Information.
    /// Current temperature in degrees Celsius.
    CurrentTemperature(Temperature) = 0x0000,
    /// Minimum temperature experienced in degrees Celsius.
    MinTempExperienced(Temperature) = 0x0001,
    /// Maximum temperature experienced in degrees Celsius.
    MaxTempExperienced(Temperature) = 0x0002,
    /// Total time the temperature was above the maximum threshold in hours.
    OverTempTotalDwell(Uint16) = 0x0003,
    // Device Temperature Settings.
    /// Alarms mask for device temperature.
    DeviceTempAlarmMask(DeviceTempAlarmMask) = 0x0010,
    /// Low temperature threshold in degrees Celsius.
    LowTempThreshold(Temperature) = 0x0011,
    /// High temperature threshold in degrees Celsius.
    HighTempThreshold(Temperature) = 0x0012,
    /// Low temperature dwell trip point in seconds.
    LowTempDwellTripPoint(Uint24) = 0x0013,
    /// High temperature dwell trip point in seconds.
    HighTempDwellTripPoint(Uint24) = 0x0014,
}

impl ToLeStream for Attribute {
    type Iter = Chain<<u16 as ToLeStream>::Iter, iterator::Attribute>;

    fn to_le_stream(self) -> Self::Iter {
        let id = self.discriminant();
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
    use le_stream::ToLeStream;

    use crate::types::{Uint16, Uint24};
    use crate::zcl::device_temperature_configuration::{DeviceTempAlarmMask, Temperature};

    pub enum Attribute {
        Temperature(<Temperature as ToLeStream>::Iter),
        Uint16(<Uint16 as ToLeStream>::Iter),
        DeviceTempAlarmMask(<DeviceTempAlarmMask as ToLeStream>::Iter),
        Uint24(<Uint24 as ToLeStream>::Iter),
    }

    impl Iterator for Attribute {
        type Item = u8;

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Self::Uint16(iter) | Self::Temperature(iter) => iter.next(),
                Self::DeviceTempAlarmMask(iter) => iter.next(),
                Self::Uint24(iter) => iter.next(),
            }
        }
    }

    impl From<Temperature> for Attribute {
        fn from(value: Temperature) -> Self {
            Self::Temperature(value.to_le_stream())
        }
    }

    impl From<Uint16> for Attribute {
        fn from(value: Uint16) -> Self {
            Self::Uint16(value.to_le_stream())
        }
    }

    impl From<DeviceTempAlarmMask> for Attribute {
        fn from(value: DeviceTempAlarmMask) -> Self {
            Self::DeviceTempAlarmMask(value.to_le_stream())
        }
    }

    impl From<Uint24> for Attribute {
        fn from(value: Uint24) -> Self {
            Self::Uint24(value.to_le_stream())
        }
    }
}
