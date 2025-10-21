use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Direction of the color loop.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum ColorLoopDirection {
    /// Increment `EnhancedCurrentHue`.
    Increment = 0x00,
    /// Decrement `EnhancedCurrentHue`.
    Decrement = 0x01,
}

impl TryFrom<u8> for ColorLoopDirection {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
