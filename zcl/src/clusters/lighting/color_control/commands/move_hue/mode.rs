//! Data structures for the `Move Hue` command in the `Lighting` cluster.

use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Move misc.
#[derive(Clone, Copy, Debug, Eq, Hash, IntoPrimitive, PartialEq, TryFromPrimitive)]
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
