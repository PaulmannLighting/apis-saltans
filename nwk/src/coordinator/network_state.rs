use std::collections::BTreeMap;

use macaddr::MacAddr8;
use zigbee::node::Node;

#[derive(Debug, Default, Clone)]
pub struct NetworkState {
    nodes: BTreeMap<u16, Node>,
}

impl NetworkState {
    /// Creates a new `NetworkState`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
        }
    }

    /// Adds a node to the network state.
    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.short_id(), node);
    }

    /// Retrieves a node by its short ID.
    pub fn get_node(&self, short_id: u16) -> Option<&Node> {
        self.nodes.get(&short_id)
    }

    /// Removes a node by its short ID.
    pub fn remove_node(&mut self, short_id: u16) -> Option<Node> {
        self.nodes.remove(&short_id)
    }

    /// Returns an iterator over all nodes in the network state.
    pub fn iter_nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.values()
    }

    /// Retrieves the short ID for a given IEEE address.
    pub fn get_short_id(&self, ieee_address: MacAddr8) -> Option<u16> {
        self.nodes
            .iter()
            .find_map(|(short_id, node)| (node.ieee_address() == ieee_address).then_some(*short_id))
    }
}
