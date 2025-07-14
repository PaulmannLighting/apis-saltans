use num_derive::FromPrimitive;

/// Step misc.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Mode {
    // 0x00 is reserved.
    /// Step up.
    Up = 0x01,
    // 0x02 is reserved.
    /// Step down.
    Down = 0x03,
}
