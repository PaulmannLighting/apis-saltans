/// Direction of the color loop.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
        match value {
            0x00 => Ok(Self::Increment),
            0x01 => Ok(Self::Decrement),
            other => Err(other),
        }
    }
}
