use le_stream::derive::{FromLeStream, ToLeStream};

/// Represents a color temperature in mireds (micro reciprocal degrees).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Mireds(u16);

impl Mireds {
    /// Minimum value for mireds.
    pub const MIN: u16 = 0x0000;
    /// Maximum value for mireds.
    pub const MAX: u16 = 0xffef;
}

impl From<Mireds> for u16 {
    fn from(value: Mireds) -> Self {
        value.0
    }
}

impl TryFrom<u16> for Mireds {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(value)
        }
    }
}
