use le_stream::ToLeStream;

use crate::types::U24;
use crate::zcl::device_temperature_configuration::DeviceTempAlarmMask;

pub enum DeviceTemperatureConfigurationAttributeIterator {
    I16(<i16 as ToLeStream>::Iter),
    U16(<u16 as ToLeStream>::Iter),
    U24(<U24 as ToLeStream>::Iter),
    DeviceTempAlarmMask(<DeviceTempAlarmMask as ToLeStream>::Iter),
}

impl Iterator for DeviceTemperatureConfigurationAttributeIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::I16(iter) | Self::U16(iter) => iter.next(),
            Self::U24(iter) => iter.next(),
            Self::DeviceTempAlarmMask(iter) => iter.next(),
        }
    }
}

impl From<i16> for DeviceTemperatureConfigurationAttributeIterator {
    fn from(value: i16) -> Self {
        Self::I16(value.to_le_stream())
    }
}

impl From<u16> for DeviceTemperatureConfigurationAttributeIterator {
    fn from(value: u16) -> Self {
        Self::U16(value.to_le_stream())
    }
}

impl From<U24> for DeviceTemperatureConfigurationAttributeIterator {
    fn from(value: U24) -> Self {
        Self::U24(value.to_le_stream())
    }
}

impl From<DeviceTempAlarmMask> for DeviceTemperatureConfigurationAttributeIterator {
    fn from(value: DeviceTempAlarmMask) -> Self {
        Self::DeviceTempAlarmMask(value.to_le_stream())
    }
}
