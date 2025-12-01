use num_derive::FromPrimitive;

/// APS frame type.
#[cfg_attr(
    feature = "le-stream",
    derive(le_stream::FromLeStream, le_stream::ToLeStream)
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, FromPrimitive)]
#[repr(u8)]
pub enum FrameType {
    /// Data frame.
    Data = 0b00,
    /// Command frame.
    Command = 0b01,
    /// Acknowledgment frame.
    Acknowledgment = 0b10,
    /// Inter-PAN APS frame.
    InterPanAps = 0b11,
}
