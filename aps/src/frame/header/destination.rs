/// Destination address for an APS frame.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Destination {
    /// Unicast endpoint address.
    Endpoint(u8),
    /// 16-bit group address.
    Group(u16),
}
