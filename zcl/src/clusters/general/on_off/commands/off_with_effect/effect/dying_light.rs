use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

/// Dying Light effect variants.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum DyingLight {
    /// 20% dim up in 0.5 seconds, then fade to off in 1 second.
    #[default]
    DimUp = 0x00,
}

impl TryFrom<u8> for DyingLight {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
