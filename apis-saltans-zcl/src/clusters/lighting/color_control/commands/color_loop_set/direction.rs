use num_derive::FromPrimitive;

/// The direction of the color loop.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Direction {
    /// Decrement the hue in the color loop.
    Decrement = 0x00,
    /// Increment the hue in the color loop.
    Increment = 0x01,
}
