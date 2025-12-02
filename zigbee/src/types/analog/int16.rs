use le_stream::{FromLeStream, ToLeStream};

const NON_VALUE: i16 = 0x8000u16.cast_signed();

/// The `16-bit signed integer` type, short `int16`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Int16(i16);

impl From<Int16> for Option<i16> {
    fn from(value: Int16) -> Self {
        value.try_into().ok()
    }
}

impl TryFrom<Int16> for i16 {
    type Error = ();

    fn try_from(value: Int16) -> Result<Self, Self::Error> {
        if value.0 == NON_VALUE {
            Err(())
        } else {
            Ok(value.0)
        }
    }
}

impl TryFrom<i16> for Int16 {
    type Error = ();

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if value == NON_VALUE {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<Option<i16>> for Int16 {
    type Error = ();

    fn try_from(value: Option<i16>) -> Result<Self, Self::Error> {
        value.map_or(Ok(Self(NON_VALUE)), Self::try_from)
    }
}
