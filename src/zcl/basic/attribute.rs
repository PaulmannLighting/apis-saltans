use core::iter::Chain;

pub use alarm_mask::AlarmMask;
pub use date_code::{CustomString, DateCode};
pub use device_enabled::DeviceEnabled;
pub use disable_local_config::DisableLocalConfig;
use le_stream::ToLeStream;
use le_stream::derive::FromLeStreamTagged;
pub use physical_environment::PhysicalEnvironment;
pub use power_source::PowerSource;
use repr_discriminant::ReprDiscriminant;

use crate::types::{String, String16, Uint8};
use crate::util::Parsable;

mod alarm_mask;
mod date_code;
mod device_enabled;
mod disable_local_config;
mod physical_environment;
mod power_source;

/// Basic Cluster Attributes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
#[derive(ReprDiscriminant, FromLeStreamTagged)]
pub enum Attribute {
    /// The ZCL version.
    ZclVersion(Uint8) = 0x0000,
    /// The application version.
    ApplicationVersion(Uint8) = 0x0001,
    /// The stack version.
    StackVersion(Uint8) = 0x0002,
    /// The hardware version.
    HwVersion(Uint8) = 0x0003,
    /// The manufacturer name.
    ManufacturerName(String) = 0x0004, // TODO: Limit to 32 bytes
    /// The model identifier.
    ModelIdentifier(String) = 0x0005, // TODO: Limit to 32 bytes
    /// The date code.
    DateCode(DateCode) = 0x0006,
    /// The power source.
    PowerSource(PowerSource) = 0x0007,
    /// The generic device class.
    LocationDescription(String16) = 0x0010,
    /// The physical environment.
    PhysicalEnvironment(PhysicalEnvironment) = 0x0011,
    /// The device enabled state.
    DeviceEnabled(Parsable<u8, DeviceEnabled>) = 0x0012,
    /// The alarm mask.
    AlarmMask(AlarmMask) = 0x0013,
    /// The disable local configuration attribute.
    DisableLocalConfig(DisableLocalConfig) = 0x0014,
    /// The cluster revision.
    SwBuildId(String16) = 0x4000,
}

impl ToLeStream for Attribute {
    type Iter = Chain<<u16 as ToLeStream>::Iter, iterator::Attribute>;

    fn to_le_stream(self) -> Self::Iter {
        let id = self.discriminant();
        let payload_iterator: iterator::Attribute = match self {
            Self::ZclVersion(value)
            | Self::ApplicationVersion(value)
            | Self::StackVersion(value)
            | Self::HwVersion(value) => value.into(),
            Self::ManufacturerName(name) => name.into(),
            Self::ModelIdentifier(identifier) => identifier.into(),
            Self::DateCode(date_code) => date_code.into(),
            Self::PowerSource(source) => source.into(),
            Self::LocationDescription(description) => description.into(),
            Self::PhysicalEnvironment(environment) => environment.into(),
            Self::DeviceEnabled(enabled) => enabled.into(),
            Self::AlarmMask(mask) => mask.into(),
            Self::DisableLocalConfig(value) => value.into(),
            Self::SwBuildId(build_id) => build_id.into(),
        };
        id.to_le_stream().chain(payload_iterator)
    }
}

/// Iterator for `Attribute` payloads.
mod iterator {
    use le_stream::ToLeStream;

    use crate::types::{String, String16, Uint8};
    use crate::util::Parsable;
    use crate::zcl::basic::{
        AlarmMask, DateCode, DeviceEnabled, DisableLocalConfig, PhysicalEnvironment, PowerSource,
    };

    /// Little endian stream iterator for the payload of an attribute in the Basic cluster.
    pub enum Attribute {
        Uint8(<Uint8 as ToLeStream>::Iter),
        String16(<String16 as ToLeStream>::Iter),
        String(<String as ToLeStream>::Iter),
        DateCode(<DateCode as ToLeStream>::Iter),
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
                Self::DateCode(iter) => iter.next(),
                Self::String(iter) => iter.next(),
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

    impl From<String16> for Attribute {
        fn from(value: String16) -> Self {
            Self::String16(value.to_le_stream())
        }
    }

    impl From<String> for Attribute {
        fn from(value: String) -> Self {
            Self::String(value.to_le_stream())
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
}
