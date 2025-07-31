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

use crate::types::{String, Uint8};
use crate::util::Parsable;

mod alarm_mask;
mod date_code;
mod device_enabled;
mod disable_local_config;
mod iterator;
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
    ManufacturerName(String<32>) = 0x0004,
    /// The model identifier.
    ModelIdentifier(String<32>) = 0x0005,
    /// The date code.
    DateCode(Parsable<String, DateCode>) = 0x0006,
    /// The power source.
    PowerSource(PowerSource) = 0x0007,
    /// The generic device class.
    LocationDescription(String<16>) = 0x0010,
    /// The physical environment.
    PhysicalEnvironment(PhysicalEnvironment) = 0x0011,
    /// The device enabled state.
    DeviceEnabled(Parsable<u8, DeviceEnabled>) = 0x0012,
    /// The alarm mask.
    AlarmMask(AlarmMask) = 0x0013,
    /// The disable local configuration attribute.
    DisableLocalConfig(DisableLocalConfig) = 0x0014,
    /// The cluster revision.
    SwBuildId(String<16>) = 0x4000,
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
            Self::ManufacturerName(string) | Self::ModelIdentifier(string) => string.into(),
            Self::DateCode(date_code) => date_code.into(),
            Self::PowerSource(source) => source.into(),
            Self::PhysicalEnvironment(environment) => environment.into(),
            Self::DeviceEnabled(enabled) => enabled.into(),
            Self::AlarmMask(mask) => mask.into(),
            Self::DisableLocalConfig(value) => value.into(),
            Self::LocationDescription(string) | Self::SwBuildId(string) => string.into(),
        };
        id.to_le_stream().chain(payload_iterator)
    }
}

#[cfg(test)]
#[cfg(feature = "alloc")]
mod tests {
    use alloc::vec;
    use alloc::vec::Vec;

    use chrono::NaiveDate;
    use le_stream::FromLeStream;

    use super::*;

    #[test]
    fn zcl_version_from_le_stream() {
        let bytes = vec![0x00, 0x00, 0x06];
        let attribute = Attribute::from_le_stream(bytes.into_iter());
        assert_eq!(
            attribute,
            Some(Attribute::ZclVersion(6.try_into().unwrap()))
        );
    }

    #[test]
    fn zcl_version_to_le_stream() {
        let attribute = Attribute::ZclVersion(6.try_into().unwrap());
        let bytes: Vec<u8> = attribute.to_le_stream().collect();
        assert_eq!(bytes, vec![0x00, 0x00, 0x06]);
    }

    #[test]
    fn application_version_from_le_stream() {
        let bytes = vec![0x01, 0x00, 0x05];
        let attribute = Attribute::from_le_stream(bytes.into_iter());
        assert_eq!(
            attribute,
            Some(Attribute::ApplicationVersion(5.try_into().unwrap()))
        );
    }

    #[test]
    fn application_version_to_le_stream() {
        let attribute = Attribute::ApplicationVersion(5.try_into().unwrap());
        let bytes: Vec<u8> = attribute.to_le_stream().collect();
        assert_eq!(bytes, vec![0x01, 0x00, 0x05]);
    }

    #[test]
    fn stack_version_from_le_stream() {
        let bytes = vec![0x02, 0x00, 0x04];
        let attribute = Attribute::from_le_stream(bytes.into_iter());
        assert_eq!(
            attribute,
            Some(Attribute::StackVersion(4.try_into().unwrap()))
        );
    }

    #[test]
    fn stack_version_to_le_stream() {
        let attribute = Attribute::StackVersion(4.try_into().unwrap());
        let bytes: Vec<u8> = attribute.to_le_stream().collect();
        assert_eq!(bytes, vec![0x02, 0x00, 0x04]);
    }

    #[test]
    fn hw_version_from_le_stream() {
        let bytes = vec![0x03, 0x00, 0x02];
        let attribute = Attribute::from_le_stream(bytes.into_iter());
        assert_eq!(attribute, Some(Attribute::HwVersion(2.try_into().unwrap())));
    }

    #[test]
    fn hw_version_to_le_stream() {
        let attribute = Attribute::HwVersion(2.try_into().unwrap());
        let bytes: Vec<u8> = attribute.to_le_stream().collect();
        assert_eq!(bytes, vec![0x03, 0x00, 0x02]);
    }

    #[test]
    fn manufacturer_name_from_le_stream() {
        let bytes = vec![0x04, 0x00, 0x04, b'T', b'e', b's', b't'];
        let attribute = Attribute::from_le_stream(bytes.into_iter());
        assert_eq!(
            attribute,
            Some(Attribute::ManufacturerName("Test".try_into().unwrap()))
        );
    }

    #[test]
    fn manufacturer_name_to_le_stream() {
        let attribute = Attribute::ManufacturerName("Test".try_into().unwrap());
        let bytes: Vec<u8> = attribute.to_le_stream().collect();
        assert_eq!(bytes, vec![0x04, 0x00, 0x04, b'T', b'e', b's', b't']);
    }

    #[test]
    fn date_code_without_custom_from_le_stream() {
        let bytes = vec![
            0x06, 0x00, 0x08, b'2', b'0', b'0', b'6', b'0', b'8', b'1', b'4',
        ];
        let attribute = Attribute::from_le_stream(bytes.into_iter());

        let Attribute::DateCode(date_code) = attribute.unwrap() else {
            panic!("Expected DateCode attribute");
        };

        let date_code = date_code.parse().unwrap();
        assert_eq!(
            date_code,
            DateCode::new(
                NaiveDate::from_ymd_opt(2006, 8, 14).unwrap(),
                CustomString::new()
            )
        );
    }

    #[test]
    fn date_code_with_custom_from_le_stream() {
        let bytes = vec![
            0x06, 0x00, 0xC, b'2', b'0', b'0', b'6', b'0', b'8', b'1', b'4', b'T', b'e', b's', b't',
        ];
        let attribute = Attribute::from_le_stream(bytes.into_iter());

        let Attribute::DateCode(date_code) = attribute.unwrap() else {
            panic!("Expected DateCode attribute");
        };

        let date_code = date_code.parse().unwrap();
        assert_eq!(
            date_code,
            DateCode::new(
                NaiveDate::from_ymd_opt(2006, 8, 14).unwrap(),
                CustomString::try_from("Test").unwrap()
            )
        );
    }

    #[test]
    fn date_code_without_custom_to_le_stream() {
        let date_code = DateCode::new(
            NaiveDate::from_ymd_opt(2006, 8, 14).unwrap(),
            CustomString::new(),
        );
        let attribute = Attribute::DateCode(date_code.into());
        let bytes: Vec<u8> = attribute.to_le_stream().collect();
        assert_eq!(
            bytes,
            vec![
                0x06, 0x00, 0x08, b'2', b'0', b'0', b'6', b'0', b'8', b'1', b'4'
            ]
        );
    }

    #[test]
    fn date_code_with_custom_to_le_stream() {
        let date_code = DateCode::new(
            NaiveDate::from_ymd_opt(2006, 8, 14).unwrap(),
            CustomString::try_from("Test").unwrap(),
        );
        let attribute = Attribute::DateCode(date_code.into());
        let bytes: Vec<u8> = attribute.to_le_stream().collect();
        assert_eq!(
            bytes,
            vec![
                0x06, 0x00, 0xC, b'2', b'0', b'0', b'6', b'0', b'8', b'1', b'4', b'T', b'e', b's',
                b't'
            ]
        );
    }

    #[test]
    fn power_source_from_le_stream() {
        let bytes = vec![0x07, 0x00, 0x03];
        let attribute = Attribute::from_le_stream(bytes.into_iter());
        assert_eq!(
            attribute,
            Some(Attribute::PowerSource(PowerSource::Battery))
        );
    }

    #[test]
    fn power_source_to_le_stream() {
        let attribute = Attribute::PowerSource(PowerSource::Battery);
        let bytes: Vec<u8> = attribute.to_le_stream().collect();
        assert_eq!(bytes, vec![0x07, 0x00, 0x03]);
    }

    #[test]
    fn location_description_from_le_stream() {
        let bytes = vec![
            0x10, 0x00, 0x08, b'L', b'o', b'c', b'a', b't', b'i', b'o', b'n',
        ];
        let attribute = Attribute::from_le_stream(bytes.into_iter());
        assert_eq!(
            attribute,
            Some(Attribute::LocationDescription(
                "Location".try_into().unwrap()
            ))
        );
    }

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
    fn physical_environment_from_le_stream() {
        let bytes = vec![0x11, 0x00, 0x02];
        let attribute = Attribute::from_le_stream(bytes.into_iter());
        assert_eq!(
            attribute,
            Some(Attribute::PhysicalEnvironment(PhysicalEnvironment::Bar))
        );
    }

    #[test]
    fn physical_environment_to_le_stream() {
        let attribute = Attribute::PhysicalEnvironment(PhysicalEnvironment::Bar);
        let bytes: Vec<u8> = attribute.to_le_stream().collect();
        assert_eq!(bytes, vec![0x11, 0x00, 0x02]);
    }

    #[test]
    fn device_enabled_from_le_stream() {
        let bytes = vec![0x12, 0x00, 0x01];
        let attribute = Attribute::from_le_stream(bytes.into_iter());
        assert_eq!(
            attribute,
            Some(Attribute::DeviceEnabled(DeviceEnabled::Enabled.into()))
        );
    }

    #[test]
    fn device_enabled_to_le_stream() {
        let attribute = Attribute::DeviceEnabled(DeviceEnabled::Enabled.into());
        let bytes: Vec<u8> = attribute.to_le_stream().collect();
        assert_eq!(bytes, vec![0x12, 0x00, 0x01]);
    }

    #[test]
    fn alarm_mask_from_le_stream() {
        let bytes = vec![0x13, 0x00, 0x01];
        let attribute = Attribute::from_le_stream(bytes.into_iter());
        assert_eq!(
            attribute,
            Some(Attribute::AlarmMask(AlarmMask::GENERAL_HARDWARE_FAULT))
        );
    }

    #[test]
    fn alarm_mask_to_le_stream() {
        let attribute = Attribute::AlarmMask(AlarmMask::GENERAL_HARDWARE_FAULT);
        let bytes: Vec<u8> = attribute.to_le_stream().collect();
        assert_eq!(bytes, vec![0x13, 0x00, 0x01]);
    }

    #[test]
    fn disable_local_config_from_le_stream() {
        let bytes = vec![0x14, 0x00, 0x01];
        let attribute = Attribute::from_le_stream(bytes.into_iter());
        assert_eq!(
            attribute,
            Some(Attribute::DisableLocalConfig(DisableLocalConfig::RESET))
        );
    }

    #[test]
    fn disable_local_config_to_le_stream() {
        let attribute = Attribute::DisableLocalConfig(DisableLocalConfig::RESET);
        let bytes: Vec<u8> = attribute.to_le_stream().collect();
        assert_eq!(bytes, vec![0x14, 0x00, 0x01]);
    }

    #[test]
    fn sw_build_id_from_le_stream() {
        let bytes = vec![0x00, 0x40, 0x04, b'T', b'e', b's', b't'];
        let attribute = Attribute::from_le_stream(bytes.into_iter());
        assert_eq!(
            attribute,
            Some(Attribute::SwBuildId("Test".try_into().unwrap()))
        );
    }

    #[test]
    fn sw_build_id_to_le_stream() {
        let attribute = Attribute::SwBuildId("Test".try_into().unwrap());
        let bytes: Vec<u8> = attribute.to_le_stream().collect();
        assert_eq!(bytes, vec![0x00, 0x40, 0x04, b'T', b'e', b's', b't']);
    }
}
