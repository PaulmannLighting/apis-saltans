use le_stream::derive::{FromLeStream, ToLeStream};

const NON_VALUE: i8 = 0x80u8.cast_signed();

/// The `8-bit signed integer` type, short `int8`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Int8(i8);

impl From<Int8> for Option<i8> {
    fn from(value: Int8) -> Self {
        value.try_into().ok()
    }
}

impl TryFrom<Int8> for i8 {
    type Error = ();

    fn try_from(value: Int8) -> Result<Self, Self::Error> {
        if value.0 == NON_VALUE {
            Err(())
        } else {
            Ok(value.0)
        }
    }
}

impl TryFrom<i8> for Int8 {
    type Error = ();

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value == NON_VALUE {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<Option<i8>> for Int8 {
    type Error = ();

    fn try_from(value: Option<i8>) -> Result<Self, Self::Error> {
        value.map_or(Ok(Self(NON_VALUE)), Self::try_from)
    }
}
