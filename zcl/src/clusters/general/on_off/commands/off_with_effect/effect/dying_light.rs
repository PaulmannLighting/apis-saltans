use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Dying Light effect variants.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, IntoPrimitive, PartialEq, TryFromPrimitive)]
#[num_enum(error_type(name = u8, constructor = core::convert::identity))]
#[repr(u8)]
pub enum DyingLight {
    /// 20% dim up in 0.5 seconds, then fade to off in 1 second.
    #[default]
    DimUp = 0x00,
}
