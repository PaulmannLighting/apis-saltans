use le_stream::ToLeStream;

use crate::types::{String16, String32};
use crate::zcl::basic::{
    AlarmMask, Attribute, DateCode, DeviceEnabled, PhysicalEnvironment, PowerSource,
};

/// Little endian stream iterator for the `Attribute` in the Basic cluster.
pub struct BasicAttributeIterator {
    prefix: <u16 as ToLeStream>::Iter,
    payload: PayloadIterator,
}

impl BasicAttributeIterator {
    fn new(id: u16, payload: PayloadIterator) -> Self {
        Self {
            prefix: id.to_le_stream(),
            payload,
        }
    }
}

impl From<Attribute> for BasicAttributeIterator {
    fn from(attribute: Attribute) -> Self {
        let id = attribute.id();
        match attribute {
            Attribute::ApplicationVersion(version)
            | Attribute::HwVersion(version)
            | Attribute::StackVersion(version)
            | Attribute::ZclVersion(version) => Self::new(id, version.into()),
            Attribute::ManufacturerName(name) => Self::new(id, name.into()),
            Attribute::ModelIdentifier(model_id) => Self::new(id, model_id.into()),
            Attribute::DateCode(date_code) => Self::new(id, date_code.into()),
            Attribute::PowerSource(power_source) => Self::new(id, power_source.into()),
            Attribute::LocationDescription(location) => Self::new(id, location.into()),
            Attribute::PhysicalEnvironment(environment) => Self::new(id, environment.into()),
            Attribute::DeviceEnabled(enabled) => Self::new(id, enabled.into()),
            Attribute::AlarmMask(alarm_mask) => Self::new(id, alarm_mask.into()),
            Attribute::DisableLocalConfig(disable) => Self::new(id, disable.into()),
            Attribute::SwBuildId(sw_build_id) => Self::new(id, sw_build_id.into()),
        }
    }
}

impl Iterator for BasicAttributeIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.prefix.next().or_else(|| self.payload.next())
    }
}

/// Little endian stream iterator for the payload of an attribute in the Basic cluster.
enum PayloadIterator {
    U8(<u8 as ToLeStream>::Iter),
    String16(<String16 as ToLeStream>::Iter),
    String32(<String32 as ToLeStream>::Iter),
    DateCode(<DateCode as ToLeStream>::Iter),
    PowerSource(<PowerSource as ToLeStream>::Iter),
    PhysicalEnvironment(<PhysicalEnvironment as ToLeStream>::Iter),
    DeviceEnabled(<DeviceEnabled as ToLeStream>::Iter),
    AlarmMask(<AlarmMask as ToLeStream>::Iter),
}

impl Iterator for PayloadIterator {
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

impl From<u8> for PayloadIterator {
    fn from(value: u8) -> Self {
        Self::U8(value.to_le_stream())
    }
}

impl From<String16> for PayloadIterator {
    fn from(value: String16) -> Self {
        Self::String16(value.to_le_stream())
    }
}

impl From<String32> for PayloadIterator {
    fn from(value: String32) -> Self {
        Self::String32(value.to_le_stream())
    }
}

impl From<DateCode> for PayloadIterator {
    fn from(value: DateCode) -> Self {
        Self::DateCode(value.to_le_stream())
    }
}

impl From<PowerSource> for PayloadIterator {
    fn from(value: PowerSource) -> Self {
        Self::PowerSource(value.to_le_stream())
    }
}

impl From<PhysicalEnvironment> for PayloadIterator {
    fn from(value: PhysicalEnvironment) -> Self {
        Self::PhysicalEnvironment(value.to_le_stream())
    }
}

impl From<DeviceEnabled> for PayloadIterator {
    fn from(value: DeviceEnabled) -> Self {
        Self::DeviceEnabled(value.to_le_stream())
    }
}

impl From<AlarmMask> for PayloadIterator {
    fn from(value: AlarmMask) -> Self {
        Self::AlarmMask(value.to_le_stream())
    }
}
