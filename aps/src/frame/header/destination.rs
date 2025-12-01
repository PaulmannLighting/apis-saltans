use le_stream::ToLeStream;

/// Destination address for an APS frame.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Destination {
    /// Unicast endpoint address.
    Unicast(u8),
    /// Broadcast address.
    Broadcast(u8),
    /// 16-bit group address.
    Group(u16),
}

impl ToLeStream for Destination {
    type Iter = Box<dyn Iterator<Item = u8>>;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::Unicast(endpoint) | Self::Broadcast(endpoint) => {
                Box::new(endpoint.to_le_stream())
            }
            Self::Group(group_addr) => Box::new(group_addr.to_le_stream()),
        }
    }
}
