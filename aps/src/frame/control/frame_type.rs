use num_derive::FromPrimitive;

/// APS aps type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, FromPrimitive)]
#[repr(u8)]
pub enum FrameType {
    /// Data aps.
    Data = 0b00,
    /// Command aps.
    Command = 0b01,
    /// Acknowledgment aps.
    Acknowledgment = 0b10,
    /// Inter-PAN APS aps.
    InterPanAps = 0b11,
}
