use core::iter::Chain;

pub use alarm_mask::AlarmMask;
pub use date_code::{CustomString, DateCode};
pub use device_enabled::DeviceEnabled;
use le_stream::ToLeStream;
use le_stream::derive::FromLeStreamTagged;
pub use physical_environment::PhysicalEnvironment;
pub use power_source::PowerSource;
use repr_discriminant::repr_discriminant;

use crate::types::{String16, String32};

mod alarm_mask;
mod date_code;
mod device_enabled;
mod physical_environment;
mod power_source;

/// Basic Cluster Attributes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr_discriminant(u16, id)]
#[derive(FromLeStreamTagged)]
pub enum Attribute {
    /// The ZCL version.
    ZclVersion(u8) = 0x0000,
    /// The application version.
    ApplicationVersion(u8) = 0x0001,
    /// The stack version.
    StackVersion(u8) = 0x0002,
    /// The hardware version.
    HwVersion(u8) = 0x0003,
    /// The manufacturer name.
    ManufacturerName(String32) = 0x0004,
    /// The model identifier.
    ModelIdentifier(String32) = 0x0005,
    /// The date code.
    DateCode(DateCode) = 0x0006,
    /// The power source.
    PowerSource(PowerSource) = 0x0007,
    /// The generic device class.
    LocationDescription(String16) = 0x0010,
    /// The physical environment.
    PhysicalEnvironment(PhysicalEnvironment) = 0x0011,
    /// The device enabled state.
    DeviceEnabled(DeviceEnabled) = 0x0012,
    /// The alarm mask.
    AlarmMask(AlarmMask) = 0x0013,
    /// The disable local configuration attribute.
    DisableLocalConfig(u8) = 0x0014,
    /// The cluster revision.
    SwBuildId(String16) = 0x4000,
}

impl ToLeStream for Attribute {
    type Iter = Chain<<u16 as ToLeStream>::Iter, iterator::Attribute>;

    fn to_le_stream(self) -> Self::Iter {
        let id = self.id();
        let payload_iterator: iterator::Attribute = match self {
            Self::ZclVersion(value)
            | Self::ApplicationVersion(value)
            | Self::StackVersion(value)
            | Self::HwVersion(value)
            | Self::DisableLocalConfig(value) => value.into(),
            Self::ManufacturerName(name) => name.into(),
            Self::ModelIdentifier(identifier) => identifier.into(),
            Self::DateCode(date_code) => date_code.into(),
            Self::PowerSource(source) => source.into(),
            Self::LocationDescription(description) => description.into(),
            Self::PhysicalEnvironment(environment) => environment.into(),
            Self::DeviceEnabled(enabled) => enabled.into(),
            Self::AlarmMask(mask) => mask.into(),
            Self::SwBuildId(build_id) => build_id.into(),
        };
        id.to_le_stream().chain(payload_iterator)
    }
}

/// Iterator for `Attribute` payloads.
mod iterator {
    use le_stream::ToLeStream;

    use crate::types::{String16, String32};
    use crate::zcl::basic::{AlarmMask, DateCode, DeviceEnabled, PhysicalEnvironment, PowerSource};

    /// Little endian stream iterator for the payload of an attribute in the Basic cluster.
    pub enum Attribute {
        U8(<u8 as ToLeStream>::Iter),
        String16(<String16 as ToLeStream>::Iter),
        String32(<String32 as ToLeStream>::Iter),
        DateCode(<DateCode as ToLeStream>::Iter),
        PowerSource(<PowerSource as ToLeStream>::Iter),
        PhysicalEnvironment(<PhysicalEnvironment as ToLeStream>::Iter),
        DeviceEnabled(<DeviceEnabled as ToLeStream>::Iter),
        AlarmMask(<AlarmMask as ToLeStream>::Iter),
    }

    impl Iterator for Attribute {
        type Item = u8;

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Self::String16(iter) | Self::DateCode(iter) => iter.next(),
                Self::String32(iter) => iter.next(),
                Self::PowerSource(iter)
                | Self::PhysicalEnvironment(iter)
                | Self::AlarmMask(iter)
                | Self::DeviceEnabled(iter)
                | Self::U8(iter) => iter.next(),
            }
        }
    }

    impl From<u8> for Attribute {
        fn from(value: u8) -> Self {
            Self::U8(value.to_le_stream())
        }
    }

    impl From<String16> for Attribute {
        fn from(value: String16) -> Self {
            Self::String16(value.to_le_stream())
        }
    }

    impl From<String32> for Attribute {
        fn from(value: String32) -> Self {
            Self::String32(value.to_le_stream())
        }
    }

    impl From<DateCode> for Attribute {
        fn from(value: DateCode) -> Self {
            Self::DateCode(value.to_le_stream())
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
}
