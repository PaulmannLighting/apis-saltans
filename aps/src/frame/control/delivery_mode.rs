use num_enum::{IntoPrimitive, TryFromPrimitive};

/// APS frame delivery mode.
#[derive(
    Clone, Copy, Debug, Eq, Hash, IntoPrimitive, Ord, PartialEq, PartialOrd, TryFromPrimitive,
)]
#[repr(u8)]
pub enum DeliveryMode {
    /// Normal unicast delivery.
    Unicast = 0b00,

    /// Acknowledgment frame.
    Broadcast = 0b10,

    /// Group addressing delivery.
    Group = 0b11,
}
