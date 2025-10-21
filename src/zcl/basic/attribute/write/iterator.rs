use le_stream::ToLeStream;
use zb::types::String;

use crate::util::Parsable;
use crate::zcl::basic::{AlarmMask, DeviceEnabled, DisableLocalConfig, PhysicalEnvironment};

/// Little endian stream iterator for the payload of an attribute in the Basic cluster.
pub enum Attribute {
    String16(<String<16> as ToLeStream>::Iter),
    PhysicalEnvironment(<PhysicalEnvironment as ToLeStream>::Iter),
    DeviceEnabled(<Parsable<u8, DeviceEnabled> as ToLeStream>::Iter),
    AlarmMask(<AlarmMask as ToLeStream>::Iter),
    DisableLocalConfig(<DisableLocalConfig as ToLeStream>::Iter),
}

impl Iterator for Attribute {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::String16(iter) => iter.next(),
            Self::PhysicalEnvironment(iter)
            | Self::AlarmMask(iter)
            | Self::DisableLocalConfig(iter)
            | Self::DeviceEnabled(iter) => iter.next(),
        }
    }
}

impl From<String<16>> for Attribute {
    fn from(value: String<16>) -> Self {
        Self::String16(value.to_le_stream())
    }
}

impl From<PhysicalEnvironment> for Attribute {
    fn from(value: PhysicalEnvironment) -> Self {
        Self::DeviceEnabled(value.to_le_stream())
    }
}

impl From<DeviceEnabled> for Attribute {
    fn from(value: DeviceEnabled) -> Self {
        Self::DeviceEnabled(value.to_le_stream())
    }
}

impl From<AlarmMask> for Attribute {
    fn from(value: AlarmMask) -> Self {
        Self::AlarmMask(value.to_le_stream())
    }
}

impl From<DisableLocalConfig> for Attribute {
    fn from(value: DisableLocalConfig) -> Self {
        Self::DisableLocalConfig(value.to_le_stream())
    }
}
