use num_derive::FromPrimitive;

/// APS frame delivery mode.
#[cfg_attr(
    feature = "le-stream",
    derive(le_stream::FromLeStream, le_stream::ToLeStream)
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, FromPrimitive)]
#[repr(u8)]
pub enum DeliveryMode {
    /// Normal unicast delivery.
    Unicast = 0b00,
    /// Acknowledgment frame.
    Broadcast = 0b10,
    /// Group addressing delivery.
    Group = 0b11,
}
