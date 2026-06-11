//! Readable attributes in the Basic cluster

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use repr_discriminant::ReprDiscriminant;
use zigbee::types::{OctStr, String, Type, Uint8};
use zigbee::{ClusterId, ClusterSpecific};

use super::alarm_mask::AlarmMask;
use super::date_code::DateCode;
use super::device_enabled::DeviceEnabled;
use super::disable_local_config::DisableLocalConfig;
use super::generic_device_class::GenericDeviceClass;
use super::generic_device_type::GenericDeviceType;
use super::physical_environment::PhysicalEnvironment;
use super::power_source::PowerSource;
use super::writable;
use crate::{InvalidType, ReadableAttribute};

/// Readable attributes in the Basic cluster.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
#[derive(ReprDiscriminant)]
pub enum Attribute {
    /// The ZCL version.
    ZclVersion(Uint8) = 0x0000,
    /// The application version.
    ApplicationVersion(Uint8) = 0x0001,
    /// The stack version.
    StackVersion(Uint8) = 0x0002,
    /// The hardware version.
    HwVersion(Uint8) = 0x0003,
    /// The manufacturer's name.
    ManufacturerName(String<32>) = 0x0004,
    /// The model identifier.
    ModelIdentifier(String<32>) = 0x0005,
    /// The date code.
    DateCode(DateCode) = 0x0006,
    /// The power source.
    PowerSource(PowerSource) = 0x0007,
    /// The generic device class.
    GenericDeviceClass(GenericDeviceClass) = 0x0008,
    /// The generic device type.
    GenericDeviceType(GenericDeviceType) = 0x0009,
    /// The product code.
    ProductCode(OctStr) = 0x000a,
    /// The product URL.
    ProductUrl(String) = 0x000b,
    /// The manufacturer version details.
    ManufacturerVersionDetails(String) = 0x000c,
    /// The serial number.
    SerialNumber(String) = 0x000d,
    /// The product label.
    ProductLabel(String) = 0x000e,
    /// The generic device class.
    LocationDescription(String<16>) = 0x0010,
    /// The physical environment.
    PhysicalEnvironment(PhysicalEnvironment) = 0x0011,
    /// The device enabled state.
    DeviceEnabled(DeviceEnabled) = 0x0012,
    /// The alarm mask.
    AlarmMask(AlarmMask) = 0x0013,
    /// Flags to disable local configuration.
    DisableLocalConfig(DisableLocalConfig) = 0x0014,
    /// The cluster revision.
    SwBuildId(String<16>) = 0x4000,
}

impl From<writable::Attribute> for Attribute {
    fn from(value: writable::Attribute) -> Self {
        match value {
            writable::Attribute::LocationDescription(string) => Self::LocationDescription(string),
            writable::Attribute::PhysicalEnvironment(environment) => {
                Self::PhysicalEnvironment(environment)
            }
            writable::Attribute::DeviceEnabled(enabled) => Self::DeviceEnabled(enabled),
            writable::Attribute::AlarmMask(mask) => Self::AlarmMask(mask),
            writable::Attribute::DisableLocalConfig(value) => Self::DisableLocalConfig(value),
        }
    }
}

impl From<Attribute> for (u16, Type) {
    fn from(value: Attribute) -> Self {
        let id = value.discriminant();
        let typ = match value {
            Attribute::ZclVersion(value)
            | Attribute::ApplicationVersion(value)
            | Attribute::StackVersion(value)
            | Attribute::HwVersion(value) => Type::Uint8(value),
            Attribute::ManufacturerName(name) | Attribute::ModelIdentifier(name) => {
                Type::String(name.widen())
            }
            Attribute::DateCode(date_code) => Type::String(String::<16>::from(date_code).widen()),
            Attribute::PowerSource(source) => Type::Enum8(source.into()),
            Attribute::GenericDeviceClass(device_class) => Type::Enum8(device_class.into()),
            Attribute::GenericDeviceType(device_type) => Type::Enum8(device_type.into()),
            Attribute::ProductCode(code) => Type::OctetString(code),
            Attribute::ProductUrl(url) => Type::String(url),
            Attribute::ManufacturerVersionDetails(details) => Type::String(details),
            Attribute::SerialNumber(serial_number) => Type::String(serial_number),
            Attribute::ProductLabel(label) => Type::String(label),
            Attribute::LocationDescription(string) => Type::String(string.widen()),
            Attribute::PhysicalEnvironment(environment) => Type::Enum8(environment.into()),
            Attribute::DeviceEnabled(enabled) => Type::Boolean(enabled.into()),
            Attribute::AlarmMask(mask) => Type::Map8(mask.bits()),
            Attribute::DisableLocalConfig(value) => Type::Map8(value.bits()),
            Attribute::SwBuildId(build_id) => Type::String(build_id.widen()),
        };

        (id, typ)
    }
}

impl TryFrom<(Id, Type)> for Attribute {
    type Error = InvalidType<Id>;

    #[expect(clippy::too_many_lines)]
    fn try_from((id, typ): (Id, Type)) -> Result<Self, Self::Error> {
        match id {
            Id::ZclVersion => {
                if let Type::Uint8(value) = typ {
                    Ok(Self::ZclVersion(value))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::ApplicationVersion => {
                if let Type::Uint8(value) = typ {
                    Ok(Self::ApplicationVersion(value))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::StackVersion => {
                if let Type::Uint8(value) = typ {
                    Ok(Self::StackVersion(value))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::HwVersion => {
                if let Type::Uint8(value) = typ {
                    Ok(Self::HwVersion(value))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::ManufacturerName => {
                if let Type::String(value) = typ {
                    match value.truncate() {
                        Ok(string) => Ok(Self::ManufacturerName(string)),
                        Err(value) => Err(InvalidType::new(id, Type::String(value))),
                    }
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::ModelIdentifier => {
                if let Type::String(value) = typ {
                    match value.truncate() {
                        Ok(string) => Ok(Self::ModelIdentifier(string)),
                        Err(value) => Err(InvalidType::new(id, Type::String(value))),
                    }
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::DateCode => {
                if let Type::String(value) = typ {
                    if let Ok(string) = value.try_as_str()
                        && let Ok(date_code) = string.parse()
                    {
                        Ok(Self::DateCode(date_code))
                    } else {
                        Err(InvalidType::new(id, Type::String(value)))
                    }
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::PowerSource => {
                if let Type::Uint8(value) = typ
                    && let Ok(value) = value.try_into()
                    && let Some(power_source) = PowerSource::from_u8(value)
                {
                    Ok(Self::PowerSource(power_source))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::GenericDeviceClass => {
                if let Type::Uint8(value) = typ
                    && let Ok(value) = value.try_into()
                    && let Some(generic_device_class) = GenericDeviceClass::from_u8(value)
                {
                    Ok(Self::GenericDeviceClass(generic_device_class))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::GenericDeviceType => {
                if let Type::Uint8(value) = typ
                    && let Ok(value) = value.try_into()
                    && let Some(generic_device_type) = GenericDeviceType::from_u8(value)
                {
                    Ok(Self::GenericDeviceType(generic_device_type))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::ProductCode => {
                if let Type::OctetString(value) = typ {
                    match value.truncate() {
                        Ok(value) => Ok(Self::ProductCode(value)),
                        Err(value) => Err(InvalidType::new(id, Type::OctetString(value))),
                    }
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::ProductUrl => {
                if let Type::String(value) = typ {
                    Ok(Self::ProductUrl(value))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::ManufacturerVersionDetails => {
                if let Type::String(value) = typ {
                    Ok(Self::ManufacturerVersionDetails(value))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::SerialNumber => {
                if let Type::String(value) = typ {
                    Ok(Self::SerialNumber(value))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::ProductLabel => {
                if let Type::String(value) = typ {
                    Ok(Self::ProductLabel(value))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::LocationDescription => {
                if let Type::String(value) = typ {
                    match value.truncate() {
                        Ok(string) => Ok(Self::LocationDescription(string)),
                        Err(value) => Err(InvalidType::new(id, Type::String(value))),
                    }
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::PhysicalEnvironment => {
                if let Type::Uint8(value) = typ
                    && let Ok(value) = value.try_into()
                    && let Some(physical_environment) = PhysicalEnvironment::from_u8(value)
                {
                    Ok(Self::PhysicalEnvironment(physical_environment))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::DeviceEnabled => {
                if let Type::Uint8(value) = typ
                    && let Ok(value) = value.try_into()
                    && let Some(device_enabled) = DeviceEnabled::from_u8(value)
                {
                    Ok(Self::DeviceEnabled(device_enabled))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::AlarmMask => {
                if let Type::Uint8(value) = typ
                    && let Ok(value) = value.try_into()
                {
                    Ok(Self::AlarmMask(AlarmMask::from_bits_retain(value)))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::DisableLocalConfig => {
                if let Type::Uint8(value) = typ
                    && let Ok(value) = value.try_into()
                {
                    Ok(Self::DisableLocalConfig(
                        DisableLocalConfig::from_bits_retain(value),
                    ))
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
            Id::SwBuildId => {
                if let Type::String(value) = typ {
                    match value.truncate() {
                        Ok(string) => Ok(Self::SwBuildId(string)),
                        Err(value) => Err(InvalidType::new(id, Type::String(value))),
                    }
                } else {
                    Err(InvalidType::new(id, typ))
                }
            }
        }
    }
}

/// IDs of readable attributes in the Basic cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive)]
#[repr(u16)]
pub enum Id {
    /// The ZCL version.
    ZclVersion = 0x0000,
    /// The application version.
    ApplicationVersion = 0x0001,
    /// The stack version.
    StackVersion = 0x0002,
    /// The hardware version.
    HwVersion = 0x0003,
    /// The manufacturer's name.
    ManufacturerName = 0x0004,
    /// The model identifier.
    ModelIdentifier = 0x0005,
    /// The date code.
    DateCode = 0x0006,
    /// The power source.
    PowerSource = 0x0007,
    /// The generic device class.
    GenericDeviceClass = 0x0008,
    /// The generic device type.
    GenericDeviceType = 0x0009,
    /// The product code.
    ProductCode = 0x000a,
    /// The product URL.
    ProductUrl = 0x000b,
    /// The manufacturer version details.
    ManufacturerVersionDetails = 0x000c,
    /// The serial number.
    SerialNumber = 0x000d,
    /// The product label.
    ProductLabel = 0x000e,
    /// The generic device class.
    LocationDescription = 0x0010,
    /// The physical environment.
    PhysicalEnvironment = 0x0011,
    /// The device enabled state.
    DeviceEnabled = 0x0012,
    /// The alarm mask.
    AlarmMask = 0x0013,
    /// Flags to disable local configuration.
    DisableLocalConfig = 0x0014,
    /// The cluster revision.
    SwBuildId = 0x4000,
}

impl ClusterSpecific for Id {
    const CLUSTER: ClusterId = ClusterId::Basic;
}

impl ReadableAttribute for Id {
    type Attribute = Attribute;
}

impl From<Id> for u16 {
    fn from(id: Id) -> Self {
        id as Self
    }
}

impl TryFrom<u16> for Id {
    type Error = u16;

    fn try_from(id: u16) -> Result<Self, Self::Error> {
        Self::from_u16(id).ok_or(id)
    }
}
