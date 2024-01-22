use std::collections::{HashMap, HashSet};
use std::time::SystemTime;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Status {
    UnsecuredJoin,
    SecuredJoin,
    UnsecuredRejoin,
    DeviceLeft,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum State {
    Online,
    Offline,
}

pub struct Node {
    ieee_address: IeeeAddress,
    network_address: NetworkAddress,
    mac_capabilities: HashSet<Capabilities>,
    node_descriptor: NodeDescriptor,
    power_descriptor: PowerDescriptor,
    last_update: SystemTime,
    associated_devices: HashSet<u8>,
    neighbors: HashSet<NeighborTable>,
    routes: HashSet<RoutingTable>,
    binding_table: HashSet<BindingTable>,
    endpoints: HashMap<u8, Endpoint>,
    endpoint_listeners: Vec<EndpointListener>,
    network_manager: NetworkManager,
    state: Option<State>,
    link_quality_statistics: LinkQualityHandler,
}
