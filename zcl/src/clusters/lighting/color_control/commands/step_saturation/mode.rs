//! Data structures for the `Step Saturation` command in the `Lighting` cluster.

use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Step misc.
#[derive(Clone, Copy, Debug, Eq, Hash, IntoPrimitive, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Mode {
    // 0x00 is reserved.
    /// Step up.
    Up = 0x01,
    // 0x02 is reserved.
    /// Step down.
    Down = 0x03,
}
