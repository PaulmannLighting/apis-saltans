use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

/// Whether the illuminance is on target.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum LevelStatus {
    /// Illuminance on target.
    OnTarget = 0x00,
    /// Illuminance below target.
    BelowTarget = 0x01,
    /// Illuminance above target.
    AboveTarget = 0x02,
}

impl TryFrom<u8> for LevelStatus {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}

impl From<LevelStatus> for u8 {
    fn from(value: LevelStatus) -> Self {
        value as Self
    }
}
