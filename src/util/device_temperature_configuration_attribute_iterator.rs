use le_stream::ToLeStream;

use crate::types::U24;

pub enum DeviceTemperatureConfigurationAttributeIterator {
    U8(<u8 as ToLeStream>::Iter),
    I16(<i16 as ToLeStream>::Iter),
    U16(<u16 as ToLeStream>::Iter),
    U24(<U24 as ToLeStream>::Iter),
}

impl Iterator for DeviceTemperatureConfigurationAttributeIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::U8(iter) => iter.next(),
            Self::I16(iter) => iter.next(),
            Self::U16(iter) => iter.next(),
            Self::U24(iter) => iter.next(),
        }
    }
}

impl From<u8> for DeviceTemperatureConfigurationAttributeIterator {
    fn from(value: u8) -> Self {
        Self::U8(value.to_le_stream())
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
