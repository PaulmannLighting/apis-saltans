use std::collections::BTreeMap;

use macaddr::MacAddr8;
use zigbee::node::Node;

#[derive(Debug, Default, Clone)]
pub struct NetworkState {
    nodes: BTreeMap<u16, Node>,
    ieee_addresses_to_pan_id: BTreeMap<MacAddr8, u16>,
}

impl NetworkState {
    /// Creates a new `NetworkState`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
            ieee_addresses_to_pan_id: BTreeMap::new(),
        }
    }

    /// Adds a node to the network state.
    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.pan_id(), node);
    }

    /// Retrieves a node by its PAN ID.
    pub fn get_node(&self, pan_id: u16) -> Option<&Node> {
        self.nodes.get(&pan_id)
    }

    /// Removes a node by its PAN ID.
    pub fn remove_node(&mut self, pan_id: u16) {
        self.nodes.remove(&pan_id);
    }

    /// Returns an iterator over all nodes in the network state.
    pub fn iter_nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.values()
    }

    /// Retrieves the PAN ID for a given IEEE address.
    pub fn get_pan_id(&self, ieee_address: &MacAddr8) -> Option<u16> {
        self.ieee_addresses_to_pan_id.get(ieee_address).copied()
    }
}
