use le_stream::{FromLeStream, ToLeStream};

use crate::types::Uint8;

/// Type to represent a number of units per second.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, FromLeStream, ToLeStream)]
pub struct UnitsPerSecond(Uint8);

impl UnitsPerSecond {
    /// Return the inner value.
    #[must_use]
    pub const fn into_inner(self) -> Uint8 {
        self.0
    }
}

impl Default for UnitsPerSecond {
    fn default() -> Self {
        Self(Uint8::NONE)
    }
}

impl TryFrom<u8> for UnitsPerSecond {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        value.try_into().map(Self).map_err(|()| value)
    }
}

impl TryFrom<Uint8> for UnitsPerSecond {
    type Error = Uint8;

    fn try_from(value: Uint8) -> Result<Self, Self::Error> {
        if value == Uint8::NONE {
            Err(value)
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<UnitsPerSecond> for u8 {
    type Error = UnitsPerSecond;

    fn try_from(value: UnitsPerSecond) -> Result<Self, Self::Error> {
        value.0.try_into().map_err(|()| value)
    }
}
