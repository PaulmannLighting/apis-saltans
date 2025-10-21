use le_stream::derive::{FromLeStream, ToLeStream};

const NON_VALUE: i32 = 0x8000_0000u32.cast_signed();

/// The `32-bit signed integer` type, short `int32`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Int32(i32);

impl From<Int32> for Option<i32> {
    fn from(value: Int32) -> Self {
        value.try_into().ok()
    }
}

impl TryFrom<Int32> for i32 {
    type Error = ();

    fn try_from(value: Int32) -> Result<Self, Self::Error> {
        if value.0 == NON_VALUE {
            Err(())
        } else {
            Ok(value.0)
        }
    }
}

impl TryFrom<i32> for Int32 {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value == NON_VALUE {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<Option<i32>> for Int32 {
    type Error = ();

    fn try_from(value: Option<i32>) -> Result<Self, Self::Error> {
        value.map_or(Ok(Self(NON_VALUE)), Self::try_from)
    }
}
