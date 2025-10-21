use le_stream::derive::{FromLeStream, ToLeStream};

const NON_VALUE: i64 = 0x8000_0000_0000_0000u64.cast_signed();

/// The `64-bit signed integer` type, short `int64`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Int64(i64);

impl From<Int64> for Option<i64> {
    fn from(value: Int64) -> Self {
        value.try_into().ok()
    }
}

impl TryFrom<Int64> for i64 {
    type Error = ();

    fn try_from(value: Int64) -> Result<Self, Self::Error> {
        if value.0 == NON_VALUE {
            Err(())
        } else {
            Ok(value.0)
        }
    }
}

impl TryFrom<i64> for Int64 {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value == NON_VALUE {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<Option<i64>> for Int64 {
    type Error = ();

    fn try_from(value: Option<i64>) -> Result<Self, Self::Error> {
        value.map_or(Ok(Self(NON_VALUE)), Self::try_from)
    }
}
