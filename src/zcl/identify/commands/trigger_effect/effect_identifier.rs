use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Available effect identifiers.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum EffectIdentifier {
    Blink = 0x00,
    Breathe = 0x01,
    Okay = 0x02,
    ChannelChange = 0x0b,
    FinishEffect = 0xfe,
    StopEffect = 0xff,
}

impl From<EffectIdentifier> for u8 {
    fn from(effect: EffectIdentifier) -> Self {
        effect as Self
    }
}

impl TryFrom<u8> for EffectIdentifier {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
