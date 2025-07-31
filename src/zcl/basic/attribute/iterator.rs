use le_stream::ToLeStream;

use crate::types::{String, Uint8};
use crate::util::Parsable;
use crate::zcl::basic::{
    AlarmMask, DateCode, DeviceEnabled, DisableLocalConfig, PhysicalEnvironment, PowerSource,
};

/// Little endian stream iterator for the payload of an attribute in the Basic cluster.
pub enum Attribute {
    Uint8(<Uint8 as ToLeStream>::Iter),
    String16(<String<16> as ToLeStream>::Iter),
    String32(<String<32> as ToLeStream>::Iter),
    PowerSource(<PowerSource as ToLeStream>::Iter),
    PhysicalEnvironment(<PhysicalEnvironment as ToLeStream>::Iter),
    DeviceEnabled(<Parsable<u8, DeviceEnabled> as ToLeStream>::Iter),
    DisableLocalConfig(<DisableLocalConfig as ToLeStream>::Iter),
    AlarmMask(<AlarmMask as ToLeStream>::Iter),
}

impl Iterator for Attribute {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::String16(iter) => iter.next(),
            Self::String32(iter) => iter.next(),
            Self::PowerSource(iter)
            | Self::PhysicalEnvironment(iter)
            | Self::AlarmMask(iter)
            | Self::DisableLocalConfig(iter)
            | Self::DeviceEnabled(iter)
            | Self::Uint8(iter) => iter.next(),
        }
    }
}

impl From<Uint8> for Attribute {
    fn from(value: Uint8) -> Self {
        Self::Uint8(value.to_le_stream())
    }
}

impl From<String<16>> for Attribute {
    fn from(value: String<16>) -> Self {
        Self::String16(value.to_le_stream())
    }
}

impl From<String<32>> for Attribute {
    fn from(value: String<32>) -> Self {
        Self::String32(value.to_le_stream())
    }
}

impl From<Parsable<String, DateCode>> for Attribute {
    fn from(value: Parsable<String, DateCode>) -> Self {
        Self::String16(value.to_le_stream())
    }
}

impl From<PowerSource> for Attribute {
    fn from(value: PowerSource) -> Self {
        Self::PowerSource(value.to_le_stream())
    }
}

impl From<PhysicalEnvironment> for Attribute {
    fn from(value: PhysicalEnvironment) -> Self {
        Self::PhysicalEnvironment(value.to_le_stream())
    }
}

impl From<Parsable<u8, DeviceEnabled>> for Attribute {
    fn from(value: Parsable<u8, DeviceEnabled>) -> Self {
        Self::DeviceEnabled(value.to_le_stream())
    }
}

impl From<DisableLocalConfig> for Attribute {
    fn from(value: DisableLocalConfig) -> Self {
        Self::DisableLocalConfig(value.to_le_stream())
    }
}

impl From<AlarmMask> for Attribute {
    fn from(value: AlarmMask) -> Self {
        Self::AlarmMask(value.to_le_stream())
    }
}
