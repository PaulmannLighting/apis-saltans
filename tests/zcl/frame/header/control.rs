use zigbee::zcl::{Control, Direction, Type};

#[test]
fn cluster_specific() {
    let control = Control::new(Type::ClusterSpecific, true, Direction::ServerToClient, true);
    assert_eq!(control.typ(), Ok(Type::ClusterSpecific));
    assert!(control.is_manufacturer_specific());
    assert_eq!(control.direction(), Direction::ServerToClient);
    assert!(control.disable_default_response());
}

#[test]
fn global() {
    let control = Control::new(Type::Global, true, Direction::ServerToClient, true);
    assert_eq!(control.typ(), Ok(Type::Global));
    assert!(control.is_manufacturer_specific());
    assert_eq!(control.direction(), Direction::ServerToClient);
    assert!(control.disable_default_response());
}

#[test]
fn manufacturer_unspecific() {
    let control = Control::new(
        Type::ClusterSpecific,
        false,
        Direction::ServerToClient,
        true,
    );
    assert_eq!(control.typ(), Ok(Type::ClusterSpecific));
    assert!(!control.is_manufacturer_specific());
    assert_eq!(control.direction(), Direction::ServerToClient);
    assert!(control.disable_default_response());
}

#[test]
fn disable_client_to_server() {
    let control = Control::new(Type::ClusterSpecific, true, Direction::ClientToServer, true);
    assert_eq!(control.typ(), Ok(Type::ClusterSpecific));
    assert!(control.is_manufacturer_specific());
    assert_eq!(control.direction(), Direction::ClientToServer);
    assert!(control.disable_default_response());
}

#[test]
fn enable_client_response() {
    let control = Control::new(
        Type::ClusterSpecific,
        true,
        Direction::ClientToServer,
        false,
    );
    assert_eq!(control.typ(), Ok(Type::ClusterSpecific));
    assert!(control.is_manufacturer_specific());
    assert_eq!(control.direction(), Direction::ClientToServer);
    assert!(!control.disable_default_response());
}
