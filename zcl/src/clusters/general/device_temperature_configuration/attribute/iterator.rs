//! Iterator over Device Temperature Configuration attributes.

use le_stream::ToLeStream;
use zigbee::types::{Uint16, Uint24};

use crate::clusters::general::device_temperature_configuration::{
    DeviceTempAlarmMask, Temperature,
};

/// Little endian stream iterator for the payload of an attribute in the Device Temperature Configuration cluster.
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
