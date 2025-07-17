use le_stream::ToLeStream;

use crate::types::String16;
use crate::zcl::power_configuration::{
    BatteryAlarmMask, BatteryAlarmState, BatterySize, MainsAlarmMask,
};

/// Little endian stream iterator for the [`Attribute`](crate::zcl::power_configuration::Attribute)
/// in the Power Configuration cluster.
pub enum PowerConfigurationAttributeIterator {
    U8(<u8 as ToLeStream>::Iter),
    U16(<u16 as ToLeStream>::Iter),
    U32(<u32 as ToLeStream>::Iter),
    String16(<String16 as ToLeStream>::Iter),
    MainsAlarmMask(<MainsAlarmMask as ToLeStream>::Iter),
    BatterySize(<BatterySize as ToLeStream>::Iter),
    BatteryAlarmMask(<BatteryAlarmMask as ToLeStream>::Iter),
    BatteryAlarmState(<BatteryAlarmState as ToLeStream>::Iter),
}

impl Iterator for PowerConfigurationAttributeIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::U8(iter) => iter.next(),
            Self::U16(iter) => iter.next(),
            Self::U32(iter) => iter.next(),
            Self::String16(iter) => iter.next(),
            Self::MainsAlarmMask(iter) => iter.next(),
            Self::BatterySize(iter) => iter.next(),
            Self::BatteryAlarmMask(iter) => iter.next(),
            Self::BatteryAlarmState(iter) => iter.next(),
        }
    }
}

impl From<u8> for PowerConfigurationAttributeIterator {
    fn from(value: u8) -> Self {
        Self::U8(value.to_le_stream())
    }
}

impl From<u16> for PowerConfigurationAttributeIterator {
    fn from(value: u16) -> Self {
        Self::U16(value.to_le_stream())
    }
}

impl From<u32> for PowerConfigurationAttributeIterator {
    fn from(value: u32) -> Self {
        Self::U32(value.to_le_stream())
    }
}

impl From<String16> for PowerConfigurationAttributeIterator {
    fn from(value: String16) -> Self {
        Self::String16(value.to_le_stream())
    }
}

impl From<MainsAlarmMask> for PowerConfigurationAttributeIterator {
    fn from(value: MainsAlarmMask) -> Self {
        Self::MainsAlarmMask(value.to_le_stream())
    }
}

impl From<BatterySize> for PowerConfigurationAttributeIterator {
    fn from(value: BatterySize) -> Self {
        Self::BatterySize(value.to_le_stream())
    }
}

impl From<BatteryAlarmMask> for PowerConfigurationAttributeIterator {
    fn from(value: BatteryAlarmMask) -> Self {
        Self::BatteryAlarmMask(value.to_le_stream())
    }
}

impl From<BatteryAlarmState> for PowerConfigurationAttributeIterator {
    fn from(value: BatteryAlarmState) -> Self {
        Self::BatteryAlarmState(value.to_le_stream())
    }
}
