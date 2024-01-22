use crate::node::Node;

pub trait Listener {
    fn node_added(&mut self, _: Node) {}

    fn node_updated(&mut self, _: Node) {}

    fn node_removed(&mut self, _: Node) {}
}
