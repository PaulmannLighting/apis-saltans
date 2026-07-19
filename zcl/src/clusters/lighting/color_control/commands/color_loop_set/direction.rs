use num_enum::{IntoPrimitive, TryFromPrimitive};

/// The direction of the color loop.
#[derive(
    Clone, Copy, Debug, Eq, Hash, IntoPrimitive, Ord, PartialEq, PartialOrd, TryFromPrimitive,
)]
#[repr(u8)]
pub enum Direction {
    /// Decrement the hue in the color loop.
    Decrement = 0x00,
    /// Increment the hue in the color loop.
    Increment = 0x01,
}
