//! Readable attributes in the Basic cluster

use le_stream::derive::FromLeStreamTagged;
use repr_discriminant::ReprDiscriminant;

use super::alarm_mask::AlarmMask;
use super::date_code::DateCode;
use super::device_enabled::DeviceEnabled;
use super::disable_local_config::DisableLocalConfig;
use super::generic_device_class::GenericDeviceClass;
use super::generic_device_type::GenericDeviceType;
use super::physical_environment::PhysicalEnvironment;
use super::power_source::PowerSource;
use crate::types::{OctStr, String, Uint8};
use crate::util::Parsable;
use crate::zcl::basic::write;

/// Readable attributes in the Basic cluster.
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
    DateCode(Parsable<String<16>, DateCode>) = 0x0006,
    /// The power source.
    PowerSource(Parsable<u8, PowerSource>) = 0x0007,
    /// The generic device class.
    GenericDeviceClass(Parsable<u8, GenericDeviceClass>) = 0x0008,
    /// The generic device type.
    GenericDeviceType(Parsable<u8, GenericDeviceType>) = 0x0009,
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
    PhysicalEnvironment(Parsable<u8, PhysicalEnvironment>) = 0x0011,
    /// The device enabled state.
    DeviceEnabled(Parsable<u8, DeviceEnabled>) = 0x0012,
    /// The alarm mask.
    AlarmMask(AlarmMask) = 0x0013,
    /// The disable local configuration attribute.
    DisableLocalConfig(DisableLocalConfig) = 0x0014,
    /// The cluster revision.
    SwBuildId(String<16>) = 0x4000,
}

impl From<write::Attribute> for Attribute {
    fn from(value: write::Attribute) -> Self {
        match value {
            write::Attribute::LocationDescription(string) => Self::LocationDescription(string),
            write::Attribute::PhysicalEnvironment(environment) => {
                Self::PhysicalEnvironment(environment.into())
            }
            write::Attribute::DeviceEnabled(enabled) => Self::DeviceEnabled(enabled.into()),
            write::Attribute::AlarmMask(mask) => Self::AlarmMask(mask),
            write::Attribute::DisableLocalConfig(value) => Self::DisableLocalConfig(value),
        }
    }
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use chrono::NaiveDate;
    use le_stream::FromLeStream;

    use super::*;
    use crate::zcl::basic::CustomString;

    #[test]
    fn zcl_version_from_le_stream() {
        let bytes = vec![0x00, 0x00, 0x06];
        let attribute = Attribute::from_le_stream(bytes.into_iter());

        let Attribute::ZclVersion(version) = attribute.unwrap() else {
            panic!("Expected ZclVersion attribute");
        };

        assert_eq!(version, 6.try_into().unwrap());
    }

    #[test]
    fn application_version_from_le_stream() {
        let bytes = vec![0x01, 0x00, 0x05];
        let attribute = Attribute::from_le_stream(bytes.into_iter());

        let Attribute::ApplicationVersion(version) = attribute.unwrap() else {
            panic!("Expected ApplicationVersion attribute");
        };

        assert_eq!(version, 5.try_into().unwrap());
    }

    #[test]
    fn stack_version_from_le_stream() {
        let bytes = vec![0x02, 0x00, 0x04];
        let attribute = Attribute::from_le_stream(bytes.into_iter());

        let Attribute::StackVersion(version) = attribute.unwrap() else {
            panic!("Expected StackVersion attribute");
        };

        assert_eq!(version, 4.try_into().unwrap());
    }

    #[test]
    fn hw_version_from_le_stream() {
        let bytes = vec![0x03, 0x00, 0x02];
        let attribute = Attribute::from_le_stream(bytes.into_iter());

        let Attribute::HwVersion(version) = attribute.unwrap() else {
            panic!("Expected HwVersion attribute");
        };

        assert_eq!(version, 2.try_into().unwrap());
    }

    #[test]
    fn manufacturer_name_from_le_stream() {
        let bytes = vec![0x04, 0x00, 0x04, b'T', b'e', b's', b't'];
        let attribute = Attribute::from_le_stream(bytes.into_iter());

        let Attribute::ManufacturerName(manufacturer) = attribute.unwrap() else {
            panic!("Expected ManufacturerName attribute");
        };

        assert_eq!(manufacturer, "Test".try_into().unwrap());
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
    fn power_source_from_le_stream() {
        let bytes = vec![0x07, 0x00, 0x03];
        let attribute = Attribute::from_le_stream(bytes.into_iter());

        let Attribute::PowerSource(power_source) = attribute.unwrap() else {
            panic!("Expected PowerSource attribute");
        };

        assert_eq!(power_source.parse(), Ok(PowerSource::Battery));
    }

    #[test]
    fn location_description_from_le_stream() {
        let bytes = vec![
            0x10, 0x00, 0x08, b'L', b'o', b'c', b'a', b't', b'i', b'o', b'n',
        ];
        let attribute = Attribute::from_le_stream(bytes.into_iter());

        let Attribute::LocationDescription(location) = attribute.unwrap() else {
            panic!("Expected LocationDescription attribute");
        };

        assert_eq!(location, "Location".try_into().unwrap());
    }

    #[test]
    fn physical_environment_from_le_stream() {
        let bytes = vec![0x11, 0x00, 0x02];
        let attribute = Attribute::from_le_stream(bytes.into_iter());

        let Attribute::PhysicalEnvironment(environment) = attribute.unwrap() else {
            panic!("Expected PhysicalEnvironment attribute");
        };

        assert_eq!(environment.parse(), Ok(PhysicalEnvironment::Bar));
    }

    #[test]
    fn device_enabled_from_le_stream() {
        let bytes = vec![0x12, 0x00, 0x01];
        let attribute = Attribute::from_le_stream(bytes.into_iter());

        let Attribute::DeviceEnabled(enabled) = attribute.unwrap() else {
            panic!("Expected DeviceEnabled attribute");
        };

        assert_eq!(enabled.parse(), Ok(DeviceEnabled::Enabled));
    }

    #[test]
    fn alarm_mask_from_le_stream() {
        let bytes = vec![0x13, 0x00, 0x01];
        let attribute = Attribute::from_le_stream(bytes.into_iter());

        let Attribute::AlarmMask(mask) = attribute.unwrap() else {
            panic!("Expected AlarmMask attribute");
        };

        assert_eq!(mask, AlarmMask::GENERAL_HARDWARE_FAULT);
    }

    #[test]
    fn disable_local_config_from_le_stream() {
        let bytes = vec![0x14, 0x00, 0x01];
        let attribute = Attribute::from_le_stream(bytes.into_iter());

        let Attribute::DisableLocalConfig(value) = attribute.unwrap() else {
            panic!("Expected DisableLocalConfig attribute");
        };

        assert_eq!(value, DisableLocalConfig::RESET);
    }

    #[test]
    fn sw_build_id_from_le_stream() {
        let bytes = vec![0x00, 0x40, 0x04, b'T', b'e', b's', b't'];
        let attribute = Attribute::from_le_stream(bytes.into_iter());

        let Attribute::SwBuildId(sw_build_id) = attribute.unwrap() else {
            panic!("Expected SwBuildId attribute");
        };

        assert_eq!(sw_build_id, "Test".try_into().unwrap());
    }
}
