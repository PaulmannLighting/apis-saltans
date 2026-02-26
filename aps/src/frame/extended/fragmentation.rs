use num_derive::FromPrimitive;

/// Fragmentation field of the APS frame header extended control field
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromPrimitive)]
#[repr(u8)]
pub enum Fragmentation {
    /// Frame is not fragmented.
    NotFragmented = 0b00,
    /// First fragment of a fragmented frame.
    FirstFragment = 0b01,
    /// More fragments to follow.
    MoreFragments = 0b10,
}
