use le_stream::derive::{FromLeStream, ToLeStream};

const NON_VALUE: u32 = 0xffff;

/// The `32-bit unsigned integer` type, short `uint32`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Uint32(u32);

impl From<Uint32> for Option<u32> {
    fn from(value: Uint32) -> Self {
        if value.0 == NON_VALUE {
            None
        } else {
            Some(value.0)
        }
    }
}

impl TryFrom<u32> for Uint32 {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == NON_VALUE {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<Option<u32>> for Uint32 {
    type Error = ();

    fn try_from(value: Option<u32>) -> Result<Self, Self::Error> {
        value.map_or(Ok(Self(NON_VALUE)), Self::try_from)
    }
}
