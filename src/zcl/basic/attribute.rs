use core::iter::Chain;

pub use alarm_mask::AlarmMask;
pub use date_code::{CustomString, DateCode};
pub use device_enabled::DeviceEnabled;
use le_stream::{FromLeStream, FromLeStreamTagged, ToLeStream};
pub use physical_environment::PhysicalEnvironment;
pub use power_source::PowerSource;
use repr_discriminant::repr_discriminant;

use crate::types::{String16, String32};
use crate::util::BasicAttributeIterator;

mod alarm_mask;
mod date_code;
mod device_enabled;
mod physical_environment;
mod power_source;

/// Basic Cluster Attributes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr_discriminant(u16, id)]
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

impl FromLeStreamTagged for Attribute {
    type Tag = u16;

    fn from_le_stream_tagged<T>(tag: Self::Tag, bytes: T) -> Result<Option<Self>, Self::Tag>
    where
        T: Iterator<Item = u8>,
    {
        match tag {
            0x0000 => Ok(u8::from_le_stream(bytes).map(Self::ZclVersion)),
            0x0001 => Ok(u8::from_le_stream(bytes).map(Self::ApplicationVersion)),
            0x0002 => Ok(u8::from_le_stream(bytes).map(Self::StackVersion)),
            0x0003 => Ok(u8::from_le_stream(bytes).map(Self::HwVersion)),
            0x0004 => Ok(String32::from_le_stream(bytes).map(Self::ManufacturerName)),
            0x0005 => Ok(String32::from_le_stream(bytes).map(Self::ModelIdentifier)),
            0x0006 => Ok(DateCode::from_le_stream(bytes).map(Self::DateCode)),
            0x0007 => Ok(PowerSource::from_le_stream(bytes).map(Self::PowerSource)),
            0x0010 => Ok(String16::from_le_stream(bytes).map(Self::LocationDescription)),
            0x0011 => Ok(PhysicalEnvironment::from_le_stream(bytes).map(Self::PhysicalEnvironment)),
            0x0012 => Ok(DeviceEnabled::from_le_stream(bytes).map(Self::DeviceEnabled)),
            0x0013 => Ok(AlarmMask::from_le_stream(bytes).map(Self::AlarmMask)),
            0x0014 => Ok(u8::from_le_stream(bytes).map(Self::DisableLocalConfig)),
            0x4000 => Ok(String16::from_le_stream(bytes).map(Self::SwBuildId)),
            unknown => Err(unknown),
        }
    }
}

impl ToLeStream for Attribute {
    type Iter = Chain<<u16 as ToLeStream>::Iter, BasicAttributeIterator>;

    fn to_le_stream(self) -> Self::Iter {
        let id = self.id();
        let payload_iterator: BasicAttributeIterator = match self {
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
