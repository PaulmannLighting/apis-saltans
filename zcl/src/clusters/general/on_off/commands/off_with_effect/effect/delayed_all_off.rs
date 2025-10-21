use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

/// Delayed all off effect variants.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum DelayedAllOff {
    /// Fade to off in 0.8 seconds.
    #[default]
    FadeToOff = 0x00,
    /// No fade.
    NoFade = 0x01,
    /// 50%% dim down in 0.8 seconds, then fade to off in 12 seconds.
    DimDown = 0x02,
}

impl TryFrom<u8> for DelayedAllOff {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
