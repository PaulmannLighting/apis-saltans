//! Readable attributes for the Device Temperature Configuration cluster.

use core::iter::Chain;

use le_stream::ToLeStream;
use le_stream::derive::FromLeStreamTagged;
use repr_discriminant::ReprDiscriminant;
use zigbee::types::{Uint16, Uint24};

use super::{iterator, write};
use crate::clusters::general::device_temperature_configuration::{
    DeviceTempAlarmMask, Temperature,
};

/// Readable attributes for the Device Temperature Configuration cluster.
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

impl From<write::Attribute> for Attribute {
    fn from(value: write::Attribute) -> Self {
        match value {
            write::Attribute::DeviceTempAlarmMask(mask) => Self::DeviceTempAlarmMask(mask),
            write::Attribute::LowTempThreshold(thresh) => Self::LowTempThreshold(thresh),
            write::Attribute::HighTempThreshold(thresh) => Self::HighTempThreshold(thresh),
            write::Attribute::LowTempDwellTripPoint(seconds) => {
                Self::LowTempDwellTripPoint(seconds)
            }
            write::Attribute::HighTempDwellTripPoint(seconds) => {
                Self::HighTempDwellTripPoint(seconds)
            }
        }
    }
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
