pub use alarm_mask::AlarmMask;
pub use date_code::DateCode;
pub use device_enabled::DeviceEnabled;
use le_stream::{FromLeStream, ToLeStream};
pub use physical_environment::PhysicalEnvironment;
pub use power_source::PowerSource;

use crate::types::{String16, String32};
use crate::util::BasicAttributeIterator;

mod alarm_mask;
mod date_code;
mod device_enabled;
mod physical_environment;
mod power_source;

/// Basic Cluster Attributes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
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

impl FromLeStream for Attribute {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        match u16::from_le_stream(&mut bytes)? {
            0x0000 => u8::from_le_stream(&mut bytes).map(Self::ZclVersion),
            0x0001 => u8::from_le_stream(&mut bytes).map(Self::ApplicationVersion),
            0x0002 => u8::from_le_stream(&mut bytes).map(Self::StackVersion),
            0x0003 => u8::from_le_stream(&mut bytes).map(Self::HwVersion),
            0x0004 => String32::from_le_stream(&mut bytes).map(Self::ManufacturerName),
            0x0005 => String32::from_le_stream(&mut bytes).map(Self::ModelIdentifier),
            0x0006 => DateCode::from_le_stream(&mut bytes).map(Self::DateCode),
            0x0007 => PowerSource::from_le_stream(&mut bytes).map(Self::PowerSource),
            0x0010 => String16::from_le_stream(&mut bytes).map(Self::LocationDescription),
            0x0011 => {
                PhysicalEnvironment::from_le_stream(&mut bytes).map(Self::PhysicalEnvironment)
            }
            0x0012 => DeviceEnabled::from_le_stream(&mut bytes).map(Self::DeviceEnabled),
            0x0013 => AlarmMask::from_le_stream(&mut bytes).map(Self::AlarmMask),
            0x0014 => u8::from_le_stream(&mut bytes).map(Self::DisableLocalConfig),
            0x4000 => String16::from_le_stream(&mut bytes).map(Self::SwBuildId),
            _ => None,
        }
    }
}

impl ToLeStream for Attribute {
    type Iter = BasicAttributeIterator;

    fn to_le_stream(self) -> Self::Iter {
        self.into()
    }
}
