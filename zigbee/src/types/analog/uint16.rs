use std::num::TryFromIntError;

use le_stream::derive::{FromLeStream, ToLeStream};

const NON_VALUE: u16 = 0xffff;

/// The `16-bit unsigned integer` type, short `uint16`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Uint16(u16);

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
