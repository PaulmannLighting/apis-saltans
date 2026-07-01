use apis_saltans_core::types::{Bool, String};
use le_stream::ToLeStream;

use crate::clusters::general::basic::{AlarmMask, DisableLocalConfig, PhysicalEnvironment};
use crate::general::basic::writable::Attribute;

/// Little endian stream iterator for the payload of an attribute in the Basic cluster.
pub enum LeStreamIter {
    String16(<String<16> as ToLeStream>::Iter),
    PhysicalEnvironment(<PhysicalEnvironment as ToLeStream>::Iter),
    DeviceEnabled(<Bool as ToLeStream>::Iter),
    AlarmMask(<AlarmMask as ToLeStream>::Iter),
    DisableLocalConfig(<DisableLocalConfig as ToLeStream>::Iter),
}

impl Iterator for LeStreamIter {
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

impl From<String<16>> for LeStreamIter {
    fn from(value: String<16>) -> Self {
        Self::String16(value.to_le_stream())
    }
}

impl From<PhysicalEnvironment> for LeStreamIter {
    fn from(value: PhysicalEnvironment) -> Self {
        Self::DeviceEnabled(value.to_le_stream())
    }
}

impl From<Bool> for LeStreamIter {
    fn from(value: Bool) -> Self {
        Self::DeviceEnabled(value.to_le_stream())
    }
}

impl From<AlarmMask> for LeStreamIter {
    fn from(value: AlarmMask) -> Self {
        Self::AlarmMask(value.to_le_stream())
    }
}

impl From<DisableLocalConfig> for LeStreamIter {
    fn from(value: DisableLocalConfig) -> Self {
        Self::DisableLocalConfig(value.to_le_stream())
    }
}

impl From<Attribute> for LeStreamIter {
    fn from(value: Attribute) -> Self {
        match value {
            Attribute::LocationDescription(string) => Self::String16(string.to_le_stream()),
            Attribute::PhysicalEnvironment(physical_environment) => {
                Self::PhysicalEnvironment(physical_environment.to_le_stream())
            }
            Attribute::DeviceEnabled(device_enabled) => {
                Self::DeviceEnabled(device_enabled.to_le_stream())
            }
            Attribute::AlarmMask(alarm_mask) => Self::AlarmMask(alarm_mask.to_le_stream()),
            Attribute::DisableLocalConfig(disable_local_config) => {
                Self::DisableLocalConfig(disable_local_config.to_le_stream())
            }
        }
    }
}
