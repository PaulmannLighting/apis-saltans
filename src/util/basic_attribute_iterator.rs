use le_stream::ToLeStream;

use crate::types::{String16, String32};
use crate::zcl::basic::{AlarmMask, DateCode, DeviceEnabled, PhysicalEnvironment, PowerSource};

/// Little endian stream iterator for the payload of an attribute in the Basic cluster.
pub enum BasicAttributeIterator {
    U8(<u8 as ToLeStream>::Iter),
    String16(<String16 as ToLeStream>::Iter),
    String32(<String32 as ToLeStream>::Iter),
    DateCode(<DateCode as ToLeStream>::Iter),
    PowerSource(<PowerSource as ToLeStream>::Iter),
    PhysicalEnvironment(<PhysicalEnvironment as ToLeStream>::Iter),
    DeviceEnabled(<DeviceEnabled as ToLeStream>::Iter),
    AlarmMask(<AlarmMask as ToLeStream>::Iter),
}

impl Iterator for BasicAttributeIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::String16(iter) | Self::DateCode(iter) => iter.next(),
            Self::String32(iter) => iter.next(),
            Self::PowerSource(iter) | Self::PhysicalEnvironment(iter) | Self::AlarmMask(iter) => {
                iter.next()
            }
            Self::DeviceEnabled(iter) | Self::U8(iter) => iter.next(),
        }
    }
}

impl From<u8> for BasicAttributeIterator {
    fn from(value: u8) -> Self {
        Self::U8(value.to_le_stream())
    }
}

impl From<String16> for BasicAttributeIterator {
    fn from(value: String16) -> Self {
        Self::String16(value.to_le_stream())
    }
}

impl From<String32> for BasicAttributeIterator {
    fn from(value: String32) -> Self {
        Self::String32(value.to_le_stream())
    }
}

impl From<DateCode> for BasicAttributeIterator {
    fn from(value: DateCode) -> Self {
        Self::DateCode(value.to_le_stream())
    }
}

impl From<PowerSource> for BasicAttributeIterator {
    fn from(value: PowerSource) -> Self {
        Self::PowerSource(value.to_le_stream())
    }
}

impl From<PhysicalEnvironment> for BasicAttributeIterator {
    fn from(value: PhysicalEnvironment) -> Self {
        Self::PhysicalEnvironment(value.to_le_stream())
    }
}

impl From<DeviceEnabled> for BasicAttributeIterator {
    fn from(value: DeviceEnabled) -> Self {
        Self::DeviceEnabled(value.to_le_stream())
    }
}

impl From<AlarmMask> for BasicAttributeIterator {
    fn from(value: AlarmMask) -> Self {
        Self::AlarmMask(value.to_le_stream())
    }
}
