use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Available effect variants.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum EffectVariant {
    Default = 0x00,
}

impl From<EffectVariant> for u8 {
    fn from(variant: EffectVariant) -> Self {
        variant as Self
    }
}

impl TryFrom<u8> for EffectVariant {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
