use crate::types::Uint8;

/// Type to represent a number of units per second.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UnitsPerSecond(Uint8);

impl Default for UnitsPerSecond {
    fn default() -> Self {
        Self(Uint8::NON_VALUE)
    }
}

impl TryFrom<u8> for UnitsPerSecond {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        value.try_into().map(Self).map_err(|()| value)
    }
}

impl TryFrom<UnitsPerSecond> for u8 {
    type Error = UnitsPerSecond;

    fn try_from(value: UnitsPerSecond) -> Result<Self, Self::Error> {
        value.0.try_into().map_err(|()| value)
    }
}
