use std::num::TryFromIntError;

use le_stream::{FromLeStream, ToLeStream};

const NON_VALUE: u16 = 0xffff;

/// The `16-bit unsigned integer` type, short `uint16`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Uint16(u16);

impl Uint16 {
    /// The minimum valid value.
    pub const MIN: Self = Self(0);

    /// The maximum valid value.
    pub const MAX: Self = Self(NON_VALUE.checked_sub(1).expect("NON_VALUE is not zero"));

    /// Crate a new `Uint16` from a raw `u16` value.
    #[must_use]
    pub const fn new(raw: u16) -> Self {
        Self(raw)
    }

    /// Convert to a `u16`.
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        self.0
    }

    /// Convert to a `usize`.
    #[must_use]
    pub const fn as_usize(self) -> usize {
        self.0 as usize
    }
}

impl From<Uint16> for Option<u16> {
    fn from(value: Uint16) -> Self {
        value.try_into().ok()
    }
}

impl TryFrom<Uint16> for u16 {
    type Error = ();

    fn try_from(value: Uint16) -> Result<Self, Self::Error> {
        if value.0 == NON_VALUE {
            Err(())
        } else {
            Ok(value.0)
        }
    }
}

impl TryFrom<u16> for Uint16 {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == NON_VALUE {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<Option<u16>> for Uint16 {
    type Error = ();

    fn try_from(value: Option<u16>) -> Result<Self, Self::Error> {
        value.map_or(Ok(Self(NON_VALUE)), Self::try_from)
    }
}

impl TryFrom<usize> for Uint16 {
    type Error = Option<TryFromIntError>;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match u16::try_from(value) {
            Ok(value) => Self::try_from(value).map_err(|()| None),
            Err(error) => Err(Some(error)),
        }
    }
}
