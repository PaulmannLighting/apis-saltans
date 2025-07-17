use le_stream::{FromLeStream, ToLeStream};
use zigbee::zcl::basic::PowerSource;

#[test]
fn power_source_from_le_stream() {
    let bytes = vec![0x01];
    let power_source = PowerSource::from_le_stream(bytes.into_iter()).unwrap();
    assert_eq!(power_source, PowerSource::MainsSinglePhase);
}

#[test]
fn power_source_to_le_stream() {
    let power_source = PowerSource::Battery;
    let bytes: Vec<u8> = power_source.to_le_stream().collect();
    assert_eq!(bytes, vec![0x03]);
}
