mod physial_environment;
mod power_source;

use chrono::NaiveDate;
use le_stream::{FromLeStream, ToLeStream};
use zigbee::types::{String, String16};
use zigbee::zcl::basic::{
    AlarmMask, Attribute, CustomString, DateCode, DeviceEnabled, DisableLocalConfig,
    PhysicalEnvironment, PowerSource,
};

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
        Some(Attribute::ManufacturerName(
            String::try_from("Test").unwrap()
        ))
    );
}

#[test]
fn manufacturer_name_to_le_stream() {
    let attribute = Attribute::ManufacturerName(String::try_from("Test").unwrap());
    let bytes: Vec<u8> = attribute.to_le_stream().collect();
    assert_eq!(bytes, vec![0x04, 0x00, 0x04, b'T', b'e', b's', b't']);
}

#[test]
fn date_code_without_custom_from_le_stream() {
    let bytes = vec![0x06, 0x00, b'2', b'0', b'0', b'6', b'0', b'8', b'1', b'4'];
    let attribute = Attribute::from_le_stream(bytes.into_iter());
    assert_eq!(
        attribute,
        Some(Attribute::DateCode(
            DateCode::new(
                NaiveDate::from_ymd_opt(2006, 8, 14).unwrap(),
                CustomString::new()
            )
            .into()
        ))
    );
}

#[test]
fn date_code_with_custom_from_le_stream() {
    let bytes = vec![
        0x06, 0x00, b'2', b'0', b'0', b'6', b'0', b'8', b'1', b'4', b'T', b'e', b's', b't',
    ];
    let attribute = Attribute::from_le_stream(bytes.into_iter());
    assert_eq!(
        attribute,
        Some(Attribute::DateCode(
            DateCode::new(
                NaiveDate::from_ymd_opt(2006, 8, 14).unwrap(),
                CustomString::try_from("Test").unwrap()
            )
            .into()
        ))
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
        vec![0x06, 0x00, b'2', b'0', b'0', b'6', b'0', b'8', b'1', b'4']
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
            0x06, 0x00, b'2', b'0', b'0', b'6', b'0', b'8', b'1', b'4', b'T', b'e', b's', b't'
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
        0x10, 0x00, 0x08, 0x00, b'L', b'o', b'c', b'a', b't', b'i', b'o', b'n',
    ];
    let attribute = Attribute::from_le_stream(bytes.into_iter());
    assert_eq!(
        attribute,
        Some(Attribute::LocationDescription(
            String16::try_from("Location").unwrap()
        ))
    );
}

#[test]
fn location_description_to_le_stream() {
    let attribute = Attribute::LocationDescription(String16::try_from("Location").unwrap());
    let bytes: Vec<u8> = attribute.to_le_stream().collect();
    assert_eq!(
        bytes,
        vec![
            0x10, 0x00, 0x08, 0x00, b'L', b'o', b'c', b'a', b't', b'i', b'o', b'n'
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
    let bytes = vec![0x00, 0x40, 0x04, 0x00, b'T', b'e', b's', b't'];
    let attribute = Attribute::from_le_stream(bytes.into_iter());
    assert_eq!(
        attribute,
        Some(Attribute::SwBuildId(String16::try_from("Test").unwrap()))
    );
}

#[test]
fn sw_build_id_to_le_stream() {
    let attribute = Attribute::SwBuildId(String16::try_from("Test").unwrap());
    let bytes: Vec<u8> = attribute.to_le_stream().collect();
    assert_eq!(bytes, vec![0x00, 0x40, 0x04, 0x00, b'T', b'e', b's', b't']);
}
