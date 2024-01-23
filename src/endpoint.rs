mod address;

use crate::node::Node;
pub use address::Address;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Endpoint {
    node: Node,
    endpoint_id: u8,
    profile_id: u8,
    device_id: u8,
    device_version: u8,
    input_clusters: HashMap<u8, Cluster>,
    output_clusters: HashMap<u8, Cluster>,
    applications: HashMap<u8, Application>,
}
