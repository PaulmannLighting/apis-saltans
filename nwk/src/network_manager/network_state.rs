use std::collections::BTreeMap;

use macaddr::MacAddr8;
use zigbee::node::Node;

#[derive(Debug, Default, Clone)]
pub struct NetworkState {
    nodes: BTreeMap<u16, Node>,
    pan_ids: BTreeMap<MacAddr8, u16>,
}

impl NetworkState {
    /// Creates a new `NetworkState`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
            pan_ids: BTreeMap::new(),
        }
    }

    /// Adds a node to the network state.
    pub fn add_node(&mut self, node: Node) {
        self.pan_ids.insert(node.ieee_address(), node.pan_id());
        self.nodes.insert(node.pan_id(), node);
    }

    /// Retrieves a node by its PAN ID.
    pub fn get_node(&self, pan_id: u16) -> Option<&Node> {
        self.nodes.get(&pan_id)
    }

    /// Removes a node by its PAN ID.
    pub fn remove_node(&mut self, pan_id: u16) {
        if let Some(node) = self.nodes.remove(&pan_id) {
            self.pan_ids.remove(&node.ieee_address());
        }
    }

    /// Returns an iterator over all nodes in the network state.
    pub fn iter_nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.values()
    }

    /// Retrieves the PAN ID for a given IEEE address.
    pub fn get_pan_id(&self, ieee_address: MacAddr8) -> Option<u16> {
        self.pan_ids.get(&ieee_address).copied()
    }
}
