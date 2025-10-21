use std::num::TryFromIntError;

use le_stream::derive::{FromLeStream, ToLeStream};

const NON_VALUE: u8 = 0xff;

/// The `8-bit unsigned integer` type, short `uint8`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Uint8(u8);

impl From<Uint8> for Option<u8> {
    fn from(value: Uint8) -> Self {
        value.try_into().ok()
    }
}

impl TryFrom<Uint8> for u8 {
    type Error = ();

    fn try_from(value: Uint8) -> Result<Self, Self::Error> {
        if value.0 == NON_VALUE {
            Err(())
        } else {
            Ok(value.0)
        }
    }
}

impl TryFrom<u8> for Uint8 {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value == NON_VALUE {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<Option<u8>> for Uint8 {
    type Error = ();

    fn try_from(value: Option<u8>) -> Result<Self, Self::Error> {
        value.map_or(Ok(Self(NON_VALUE)), Self::try_from)
    }
}

impl TryFrom<usize> for Uint8 {
    type Error = Option<TryFromIntError>;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match u8::try_from(value) {
            Ok(value) => Self::try_from(value).map_err(|()| None),
            Err(error) => Err(Some(error)),
        }
    }
}
