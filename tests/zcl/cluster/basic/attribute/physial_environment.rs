use le_stream::{FromLeStream, ToLeStream};
use zigbee::zcl::basic::PhysicalEnvironment;

#[test]
fn mirror_or_atrium_from_le_stream() {
    let bytes = vec![0x01];
    let environment = PhysicalEnvironment::from_le_stream(bytes.into_iter()).unwrap();
    assert_eq!(environment, PhysicalEnvironment::MirrorOrAtrium);
}

#[test]
fn bar_to_le_stream() {
    let environment = PhysicalEnvironment::Bar;
    let bytes: Vec<u8> = environment.to_le_stream().collect();
    assert_eq!(bytes, vec![0x02]);
}

#[test]
fn unknown() {
    let bytes = vec![0xff];
    let environment = PhysicalEnvironment::from_le_stream(bytes.into_iter()).unwrap();
    assert_eq!(environment, PhysicalEnvironment::Unknown);
}

#[test]
fn any_office_from_le_stream() {
    let mut bytes = vec![0x0b, 0x24].into_iter();
    let office = PhysicalEnvironment::from_le_stream(&mut bytes).unwrap();
    assert_eq!(office, PhysicalEnvironment::Office);
    let office_alt = PhysicalEnvironment::from_le_stream(&mut bytes).unwrap();
    assert_eq!(office_alt, PhysicalEnvironment::OfficeAlt);
}

#[test]
fn any_living_room_from_le_stream() {
    let mut bytes = vec![0x2e, 0x39].into_iter();
    let living_room = PhysicalEnvironment::from_le_stream(&mut bytes).unwrap();
    assert_eq!(living_room, PhysicalEnvironment::LivingRoom);
    let living_room_alt = PhysicalEnvironment::from_le_stream(&mut bytes).unwrap();
    assert_eq!(living_room_alt, PhysicalEnvironment::LivingRoomAlt);
}
