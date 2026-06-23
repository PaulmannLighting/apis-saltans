//! Readable attributes in the Basic cluster

use core::fmt::{Display, LowerHex, UpperHex};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use repr_discriminant::ReprDiscriminant;
use zigbee::types::{Bool, OctStr, String, Type, Uint8};
use zigbee::{ClusterId, ClusterSpecific};

use super::alarm_mask::AlarmMask;
use super::date_code::DateCode;
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
    DeviceEnabled(Bool) = 0x0012,

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
            writable::Attribute::LocationDescription(string) => Self::LocationDescription(*string),
            writable::Attribute::PhysicalEnvironment(environment) => {
                Self::PhysicalEnvironment(environment)
            }
            writable::Attribute::DeviceEnabled(enabled) => Self::DeviceEnabled(enabled),
            writable::Attribute::AlarmMask(mask) => Self::AlarmMask(mask),
            writable::Attribute::DisableLocalConfig(value) => Self::DisableLocalConfig(value),
        }
    }
}

impl From<Attribute> for Type {
    fn from(attribute: Attribute) -> Self {
        match attribute {
            Attribute::ZclVersion(value)
            | Attribute::ApplicationVersion(value)
            | Attribute::StackVersion(value)
            | Attribute::HwVersion(value) => value.into(),
            Attribute::ManufacturerName(name) | Attribute::ModelIdentifier(name) => name.into(),
            Attribute::DateCode(date_code) => date_code.into(),
            Attribute::PowerSource(power_source) => power_source.into(),
            Attribute::GenericDeviceClass(device_class) => device_class.into(),
            Attribute::GenericDeviceType(device_type) => device_type.into(),
            Attribute::ProductCode(code) => code.into(),
            Attribute::ProductUrl(url) => url.into(),
            Attribute::ManufacturerVersionDetails(details) => details.into(),
            Attribute::SerialNumber(serial_number) => serial_number.into(),
            Attribute::ProductLabel(label) => label.into(),
            Attribute::LocationDescription(string) => string.into(),
            Attribute::PhysicalEnvironment(environment) => environment.into(),
            Attribute::DeviceEnabled(enabled) => enabled.into(),
            Attribute::AlarmMask(mask) => mask.into(),
            Attribute::DisableLocalConfig(value) => value.into(),
            Attribute::SwBuildId(build_id) => build_id.into(),
        }
    }
}

impl From<Attribute> for (u16, Type) {
    fn from(attribute: Attribute) -> Self {
        let id = attribute.discriminant();
        (id, attribute.into())
    }
}

impl TryFrom<(Id, Type)> for Attribute {
    type Error = InvalidType<Id>;

    fn try_from((id, typ): (Id, Type)) -> Result<Self, Self::Error> {
        match id {
            Id::ZclVersion => typ.try_into().map(Self::ZclVersion),
            Id::ApplicationVersion => typ.try_into().map(Self::ApplicationVersion),
            Id::StackVersion => typ.try_into().map(Self::StackVersion),
            Id::HwVersion => typ.try_into().map(Self::HwVersion),
            Id::ManufacturerName => typ.try_into().map(Self::ManufacturerName),
            Id::ModelIdentifier => typ.try_into().map(Self::ModelIdentifier),
            Id::DateCode => typ.try_into().map(Self::DateCode),
            Id::PowerSource => typ.try_into().map(Self::PowerSource),
            Id::GenericDeviceClass => typ.try_into().map(Self::GenericDeviceClass),
            Id::GenericDeviceType => typ.try_into().map(Self::GenericDeviceType),
            Id::ProductCode => typ.try_into().map(Self::ProductCode),
            Id::ProductUrl => typ.try_into().map(Self::ProductUrl),
            Id::ManufacturerVersionDetails => typ.try_into().map(Self::ManufacturerVersionDetails),
            Id::SerialNumber => typ.try_into().map(Self::SerialNumber),
            Id::ProductLabel => typ.try_into().map(Self::ProductLabel),
            Id::LocationDescription => typ.try_into().map(Self::LocationDescription),
            Id::PhysicalEnvironment => typ.try_into().map(Self::PhysicalEnvironment),
            Id::DeviceEnabled => typ.try_into().map(Self::DeviceEnabled),
            Id::AlarmMask => typ.try_into().map(Self::AlarmMask),
            Id::DisableLocalConfig => typ.try_into().map(Self::DisableLocalConfig),
            Id::SwBuildId => typ.try_into().map(Self::SwBuildId),
        }
        .map_err(|typ| InvalidType::new(id, typ))
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

impl Display for Id {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl LowerHex for Id {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        LowerHex::fmt(&u16::from(*self), f)
    }
}

impl UpperHex for Id {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        UpperHex::fmt(&u16::from(*self), f)
    }
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
