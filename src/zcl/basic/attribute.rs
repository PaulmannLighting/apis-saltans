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
use crate::util::BasicAttributeIterator;

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
