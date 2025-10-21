//! Writable attributes in the Basic cluster.

use core::iter::Chain;

use le_stream::ToLeStream;
use repr_discriminant::ReprDiscriminant;
use zb::types::String;

use super::alarm_mask::AlarmMask;
use super::device_enabled::DeviceEnabled;
use super::disable_local_config::DisableLocalConfig;
use super::physical_environment::PhysicalEnvironment;
use crate::basic::read;

mod iterator;

/// Writable attributes in the Basic cluster.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
#[derive(ReprDiscriminant)]
#[allow(variant_size_differences)]
pub enum Attribute {
    /// The generic device class.
    LocationDescription(String<16>) = 0x0010,
    /// The physical environment.
    PhysicalEnvironment(PhysicalEnvironment) = 0x0011,
    /// The device enabled state.
    DeviceEnabled(DeviceEnabled) = 0x0012,
    /// The alarm mask.
    AlarmMask(AlarmMask) = 0x0013,
    /// The disable local configuration attribute.
    DisableLocalConfig(DisableLocalConfig) = 0x0014,
}

impl TryFrom<read::Attribute> for Attribute {
    type Error = read::Attribute;

    fn try_from(value: read::Attribute) -> Result<Self, Self::Error> {
        match value {
            read::Attribute::LocationDescription(string) => Ok(Self::LocationDescription(string)),
            read::Attribute::PhysicalEnvironment(parsable) => parsable.parse().map_or_else(
                |_| Err(read::Attribute::PhysicalEnvironment(parsable)),
                |physical_environment| Ok(Self::PhysicalEnvironment(physical_environment)),
            ),
            read::Attribute::DeviceEnabled(parsable) => parsable.parse().map_or_else(
                |_| Err(read::Attribute::DeviceEnabled(parsable)),
                |device_enabled| Ok(Self::DeviceEnabled(device_enabled)),
            ),
            read::Attribute::AlarmMask(mask) => Ok(Self::AlarmMask(mask)),
            read::Attribute::DisableLocalConfig(value) => Ok(Self::DisableLocalConfig(value)),
            other => Err(other),
        }
    }
}

impl ToLeStream for Attribute {
    type Iter = Chain<<u16 as ToLeStream>::Iter, iterator::Attribute>;

    fn to_le_stream(self) -> Self::Iter {
        let id = self.discriminant();
        let payload_iterator: iterator::Attribute = match self {
            Self::LocationDescription(string) => string.into(),
            Self::PhysicalEnvironment(environment) => environment.into(),
            Self::DeviceEnabled(enabled) => enabled.into(),
            Self::AlarmMask(mask) => mask.into(),
            Self::DisableLocalConfig(value) => value.into(),
        };
        id.to_le_stream().chain(payload_iterator)
    }
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use super::*;

    #[test]
    fn location_description_to_le_stream() {
        let attribute = Attribute::LocationDescription("Location".try_into().unwrap());
        let bytes: Vec<u8> = attribute.to_le_stream().collect();
        assert_eq!(
            bytes,
            vec![
                0x10, 0x00, 0x08, b'L', b'o', b'c', b'a', b't', b'i', b'o', b'n'
            ]
        );
    }

    #[test]
    fn physical_environment_to_le_stream() {
        let attribute = Attribute::PhysicalEnvironment(PhysicalEnvironment::Bar);
        let bytes: Vec<u8> = attribute.to_le_stream().collect();
        assert_eq!(bytes, vec![0x11, 0x00, 0x02]);
    }

    #[test]
    fn device_enabled_to_le_stream() {
        let attribute = Attribute::DeviceEnabled(DeviceEnabled::Enabled.into());
        let bytes: Vec<u8> = attribute.to_le_stream().collect();
        assert_eq!(bytes, vec![0x12, 0x00, 0x01]);
    }

    #[test]
    fn alarm_mask_to_le_stream() {
        let attribute = Attribute::AlarmMask(AlarmMask::GENERAL_HARDWARE_FAULT);
        let bytes: Vec<u8> = attribute.to_le_stream().collect();
        assert_eq!(bytes, vec![0x13, 0x00, 0x01]);
    }

    #[test]
    fn disable_local_config_to_le_stream() {
        let attribute = Attribute::DisableLocalConfig(DisableLocalConfig::RESET);
        let bytes: Vec<u8> = attribute.to_le_stream().collect();
        assert_eq!(bytes, vec![0x14, 0x00, 0x01]);
    }
}
