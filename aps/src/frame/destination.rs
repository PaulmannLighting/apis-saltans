#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Destination {
    /// A unicast endpoint ID.
    Unicast(u8),
    /// A broadcast endpoint ID.
    Broadcast(u8),
    /// A group address.
    Group(u16),
}
