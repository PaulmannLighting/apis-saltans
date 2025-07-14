use num_derive::FromPrimitive;

/// Move misc.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Mode {
    /// Stop move.
    Stop = 0x00,
    /// Move up.
    Up = 0x01,
    // 0x02 is reserved.
    /// Move down.
    Down = 0x03,
}
