use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Available effect identifiers.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum EffectIdentifier {
    /// Turn light off and on again.
    Blink = 0x00,
    /// Repeatedly turn light off and on again with a longer timing.
    Breathe = 0x01,
    /// Turn light green for one second or flash twice.
    Okay = 0x02,
    /// Change color channel or brightness for some time.
    ChannelChange = 0x0b,
    /// Complete the current effect.
    FinishEffect = 0xfe,
    /// Stop the current effect immediately.
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
