use le_stream::{FromLeStream, ToLeStream};
use zigbee::zcl::basic::DeviceEnabled;

#[test]
fn enabled_from_le_stream() {
    let bytes = vec![0x01];
    let device_enabled = DeviceEnabled::from_le_stream(bytes.into_iter()).unwrap();
    assert_eq!(device_enabled, DeviceEnabled::Enabled);
}

#[test]
fn disabled_from_le_stream() {
    let bytes = vec![0x00];
    let device_enabled = DeviceEnabled::from_le_stream(bytes.into_iter()).unwrap();
    assert_eq!(device_enabled, DeviceEnabled::Disabled);
}

#[test]
fn from_le_stream_invalid() {
    let bytes = vec![0x02];
    let device_enabled = DeviceEnabled::from_le_stream(bytes.into_iter());
    assert!(device_enabled.is_none());
}

#[test]
fn enabled_to_le_stream() {
    let device_enabled = DeviceEnabled::Enabled;
    let bytes: Vec<u8> = device_enabled.to_le_stream().collect();
    assert_eq!(bytes, vec![0x01]);
}

#[test]
fn disabled_to_le_stream() {
    let device_enabled = DeviceEnabled::Disabled;
    let bytes: Vec<u8> = device_enabled.to_le_stream().collect();
    assert_eq!(bytes, vec![0x00]);
}
