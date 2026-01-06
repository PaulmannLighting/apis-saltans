use num_derive::FromPrimitive;

/// APS aps delivery mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, FromPrimitive)]
#[repr(u8)]
pub enum DeliveryMode {
    /// Normal unicast delivery.
    Unicast = 0b00,
    /// Acknowledgment aps.
    Broadcast = 0b10,
    /// Group addressing delivery.
    Group = 0b11,
}
