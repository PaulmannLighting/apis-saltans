use le_stream::{FromLeStream, ToLeStream};

const NON_VALUE: u64 = 0xffff_ffff_ffff_ffff;

/// The `64-bit unsigned integer` type, short `uint64`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Uint64(u64);

impl From<Uint64> for Option<u64> {
    fn from(value: Uint64) -> Self {
        value.try_into().ok()
    }
}

impl TryFrom<Uint64> for u64 {
    type Error = ();

    fn try_from(value: Uint64) -> Result<Self, Self::Error> {
        if value.0 == NON_VALUE {
            Err(())
        } else {
            Ok(value.0)
        }
    }
}

impl TryFrom<u64> for Uint64 {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value == NON_VALUE {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<Option<u64>> for Uint64 {
    type Error = ();

    fn try_from(value: Option<u64>) -> Result<Self, Self::Error> {
        value.map_or(Ok(Self(NON_VALUE)), Self::try_from)
    }
}
