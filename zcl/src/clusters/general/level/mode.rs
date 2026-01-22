use num_derive::FromPrimitive;

/// Move mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, FromPrimitive)]
#[repr(u8)]
pub enum Mode {
    /// Move up.
    Up = 0x00,
    /// Move down.
    Down = 0x01,
}
