use num_derive::FromPrimitive;

/// Direction of hue flow.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Direction {
    /// Take the shortest distance.
    ShortestDistance = 0x00,
    /// Take the longest distance.
    LongestDistance = 0x01,
    /// Move up.
    Up = 0x02,
    /// Move down.
    Down = 0x03,
}
