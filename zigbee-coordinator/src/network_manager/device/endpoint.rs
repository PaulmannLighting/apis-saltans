use zigbee::Profile;

/// An endpoint of a device.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Endpoint {
    id: zigbee::Endpoint,
    profile_id: u16,
    device_id: u16,
    clusters: Vec<zigbee::ClusterId>,
}
