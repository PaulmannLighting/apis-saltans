use le_stream::{FromLeStream, ToLeStream};
use zigbee::types::U24;

#[test]
fn from_le_stream() {
    let bytes = vec![0x01, 0x02, 0x03];
    let value = U24::from_le_stream(bytes.into_iter());
    assert_eq!(value, Some(U24::new(0x030201).unwrap()));
}

#[test]
fn to_le_stream() {
    let value = U24::new(0x030201).unwrap();
    let bytes: Vec<u8> = value.to_le_stream().collect();
    assert_eq!(bytes, vec![0x01, 0x02, 0x03]);
}
